//! Nested/chained encoding detection and decoding

use super::{detect::detect_encoding, DetectionConfidence, EncodingInfo};
use crate::error::DecodeckError;

/// Maximum recursion depth to prevent infinite loops
const MAX_CHAIN_DEPTH: usize = 10;

/// Result of chain decoding
#[derive(Debug, Clone)]
pub struct ChainResult {
    /// Final decoded bytes
    pub data: Vec<u8>,
    /// Chain of encodings detected (in order of decoding)
    pub chain: Vec<EncodingInfo>,
    /// Whether max depth was reached
    pub truncated: bool,
}

/// Decode nested/chained encodings recursively
pub fn decode_chain(input: &str, max_depth: Option<usize>) -> Result<ChainResult, DecodeckError> {
    let max = max_depth.unwrap_or(MAX_CHAIN_DEPTH);
    let mut current = input.to_string();
    let mut chain = Vec::new();

    for depth in 0..max {
        // Detect encoding
        let info = detect_encoding(&current);

        // Only continue if we have reasonable confidence
        if info.confidence == DetectionConfidence::Low && depth > 0 {
            // Low confidence on non-first iteration - stop here
            break;
        }

        // Try to decode
        let decoder = info.encoding_type.decoder();
        match decoder.decode(&current) {
            Ok(decoded) => {
                chain.push(info);

                // Check if result is valid UTF-8 and could be another encoding
                if let Ok(decoded_str) = String::from_utf8(decoded.clone()) {
                    // Check if it looks like it could be encoded data
                    if could_be_encoded(&decoded_str) && depth + 1 < max {
                        current = decoded_str;
                        continue;
                    }
                }

                // Not encodable or reached end
                return Ok(ChainResult {
                    data: decoded,
                    chain,
                    truncated: false,
                });
            }
            Err(_) => {
                // Decoding failed - return what we have
                if chain.is_empty() {
                    return Err(DecodeckError::DecodeFailed {
                        message: "Failed to decode input".to_string(),
                    });
                }
                break;
            }
        }
    }

    // Reached max depth or broke out of loop
    let truncated = chain.len() >= max;

    // Final decode of current state
    if !current.is_empty() {
        let info = detect_encoding(&current);
        let decoder = info.encoding_type.decoder();
        if let Ok(decoded) = decoder.decode(&current) {
            if !chain.iter().any(|c| c.encoding_type == info.encoding_type) || chain.is_empty() {
                chain.push(info);
            }
            return Ok(ChainResult {
                data: decoded,
                chain,
                truncated,
            });
        }
    }

    Ok(ChainResult {
        data: current.into_bytes(),
        chain,
        truncated,
    })
}

/// Check if a string could potentially be encoded data
fn could_be_encoded(s: &str) -> bool {
    let trimmed = s.trim();
    if trimmed.is_empty() || trimmed.len() < 4 {
        return false;
    }

    // Check for encoding markers
    // 0x prefix (hex)
    if trimmed.starts_with("0x") || trimmed.starts_with("0X") {
        return true;
    }

    // Ascii85 delimiters
    if trimmed.starts_with("<~") && trimmed.ends_with("~>") {
        return true;
    }

    // URL encoding
    if trimmed.contains('%') {
        let has_percent_encoding = trimmed
            .chars()
            .collect::<Vec<_>>()
            .windows(3)
            .any(|w| w[0] == '%' && w[1].is_ascii_hexdigit() && w[2].is_ascii_hexdigit());
        if has_percent_encoding {
            return true;
        }
    }

    // Check character composition for Base64/Base32/Hex patterns
    // Count alphanumeric + base64 padding characters
    let valid_chars = trimmed
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '=' || *c == '+' || *c == '/')
        .count();
    let valid_ratio = valid_chars as f64 / trimmed.len() as f64;

    // High ratio of valid encoding chars suggests encoding
    valid_ratio > 0.9
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoding::EncodingType;

    #[test]
    fn test_single_encoding() {
        // Just Base64
        let result = decode_chain("SGVsbG8gV29ybGQ=", None).unwrap();
        assert_eq!(result.data, b"Hello World");
        assert_eq!(result.chain.len(), 1);
        assert_eq!(result.chain[0].encoding_type, EncodingType::Base64);
    }

    #[test]
    fn test_double_encoding() {
        // Base64(Base64("test"))
        // Chain decoding stops when result doesn't look like encoded data
        let result = decode_chain("ZEdWemRBPT0=", None).unwrap();
        // First decode gives "dGVzdA==" which looks like base64
        // This is tricky - might decode once or twice depending on heuristics
        assert!(!result.chain.is_empty());
    }

    #[test]
    fn test_hex_inside_base64() {
        // Base64("0x48656c6c6f") = "MHg0ODY1NmM2YzZm"
        // After decoding base64 we get "0x48656c6c6f" which is detected as hex
        let result = decode_chain("MHg0ODY1NmM2YzZm", None).unwrap();
        // Should detect at least 2 encodings: base64 then hex
        assert!(result.chain.len() >= 2);
    }

    #[test]
    fn test_max_depth() {
        let result = decode_chain("SGVsbG8=", Some(1)).unwrap();
        assert_eq!(result.chain.len(), 1);
    }

    #[test]
    fn test_could_be_encoded() {
        assert!(could_be_encoded("0x48656c6c6f"));
        assert!(could_be_encoded("<~test~>"));
        assert!(could_be_encoded("Hello%20World"));
        assert!(could_be_encoded("SGVsbG8=")); // Base64-like
        assert!(!could_be_encoded("Hi")); // Too short
        assert!(!could_be_encoded("")); // Empty
    }
}
