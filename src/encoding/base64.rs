//! Base64 encoding decoder

use super::Decoder;
use crate::error::DecodeckError;
use base64::Engine;

/// Base64 decoder implementation
pub struct Base64Decoder;

impl Decoder for Base64Decoder {
    fn decode(&self, input: &str) -> Result<Vec<u8>, DecodeckError> {
        // Strip whitespace
        let cleaned: String = input.chars().filter(|c| !c.is_whitespace()).collect();

        if cleaned.is_empty() {
            return Err(DecodeckError::NoInput);
        }

        // Detect variant and add padding
        let is_urlsafe = cleaned.contains('-') || cleaned.contains('_');
        let padded = add_padding(&cleaned);

        // Decode
        let result = if is_urlsafe {
            base64::engine::general_purpose::URL_SAFE.decode(&padded)
        } else {
            base64::engine::general_purpose::STANDARD.decode(&padded)
        };

        result.map_err(|e| DecodeckError::DecodeFailed {
            message: format!("Invalid Base64: {}", e),
        })
    }

    fn name(&self) -> &'static str {
        "base64"
    }

    fn can_decode(&self, input: &str) -> bool {
        let cleaned: String = input.chars().filter(|c| !c.is_whitespace()).collect();
        if cleaned.is_empty() {
            return false;
        }

        // Base64 characters: A-Za-z0-9+/=  or URL-safe: A-Za-z0-9-_=
        cleaned.chars().all(|c| {
            c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '-' || c == '_' || c == '='
        })
    }
}

/// Add padding to Base64 string if needed
fn add_padding(input: &str) -> String {
    let remainder = input.len() % 4;
    if remainder == 0 {
        input.to_string()
    } else {
        let padding = 4 - remainder;
        format!("{}{}", input, "=".repeat(padding))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_simple() {
        let decoder = Base64Decoder;
        let result = decoder.decode("SGVsbG8=").unwrap();
        assert_eq!(result, b"Hello");
    }

    #[test]
    fn test_decode_urlsafe() {
        let decoder = Base64Decoder;
        let result = decoder.decode("SGVsbG8tV29ybGRf").unwrap();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_can_decode() {
        let decoder = Base64Decoder;
        assert!(decoder.can_decode("SGVsbG8="));
        assert!(decoder.can_decode("SGVsbG8"));
        assert!(!decoder.can_decode("")); // empty
    }
}
