//! Auto-detection of encoding type

use super::{DetectionConfidence, EncodingInfo, EncodingType};

/// Detect the encoding type from input data
pub fn detect_encoding(input: &str) -> EncodingInfo {
    let trimmed = input.trim();

    // 1. Check for 0x prefix (definite hex)
    if trimmed.starts_with("0x") || trimmed.starts_with("0X") {
        return EncodingInfo::detected(EncodingType::Hex, DetectionConfidence::High);
    }

    // 2. Check for Ascii85 delimiters
    if trimmed.starts_with("<~") && trimmed.ends_with("~>") {
        return EncodingInfo::detected(EncodingType::Base85, DetectionConfidence::High);
    }

    // 3. Check for URL percent-encoding
    if contains_percent_encoding(trimmed) {
        return EncodingInfo::detected(EncodingType::Url, DetectionConfidence::High);
    }

    // 4. Check if it's valid hex (only hex chars, even length)
    let cleaned: String = trimmed.chars().filter(|c| !c.is_whitespace()).collect();
    if is_likely_hex(&cleaned) {
        return EncodingInfo::detected(EncodingType::Hex, DetectionConfidence::Medium);
    }

    // 5. Check if it's valid Base32 (only A-Z, 2-7)
    if is_likely_base32(&cleaned) {
        return EncodingInfo::detected(EncodingType::Base32, DetectionConfidence::Medium);
    }

    // 6. Default to Base64 (most common)
    EncodingInfo::detected(EncodingType::Base64, DetectionConfidence::Low)
}

/// Check if input contains valid percent-encoded sequences
fn contains_percent_encoding(input: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();
    for i in 0..chars.len() {
        if chars[i] == '%'
            && i + 2 < chars.len()
            && chars[i + 1].is_ascii_hexdigit()
            && chars[i + 2].is_ascii_hexdigit()
        {
            return true;
        }
    }
    false
}

/// Check if input is likely hexadecimal
fn is_likely_hex(input: &str) -> bool {
    if input.is_empty() || input.len() % 2 != 0 {
        return false;
    }

    // Must be all hex chars
    if !input.chars().all(|c| c.is_ascii_hexdigit()) {
        return false;
    }

    // Hex strings are often longer and contain both digits and letters
    // Short strings that are all digits might be numbers, not hex
    if input.len() >= 8 {
        let has_letters = input.chars().any(|c| c.is_ascii_alphabetic());
        let has_digits = input.chars().any(|c| c.is_ascii_digit());
        return has_letters || has_digits;
    }

    // For shorter strings, require at least one letter to be confident it's hex
    input.chars().any(|c| c.is_ascii_alphabetic())
}

/// Check if input is likely Base32
fn is_likely_base32(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }

    // Base32 uses A-Z and 2-7 (no 0, 1, 8, 9)
    let upper: String = input.to_uppercase();
    upper
        .chars()
        .all(|c| matches!(c, 'A'..='Z' | '2'..='7' | '='))
        && !upper.chars().any(|c| matches!(c, '0' | '1' | '8' | '9'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_hex_with_prefix() {
        let info = detect_encoding("0x48656c6c6f");
        assert_eq!(info.encoding_type, EncodingType::Hex);
        assert_eq!(info.confidence, DetectionConfidence::High);
    }

    #[test]
    fn test_detect_ascii85() {
        let info = detect_encoding("<~87cURD]j~>");
        assert_eq!(info.encoding_type, EncodingType::Base85);
        assert_eq!(info.confidence, DetectionConfidence::High);
    }

    #[test]
    fn test_detect_url() {
        let info = detect_encoding("Hello%20World");
        assert_eq!(info.encoding_type, EncodingType::Url);
        assert_eq!(info.confidence, DetectionConfidence::High);
    }

    #[test]
    fn test_detect_hex_no_prefix() {
        let info = detect_encoding("48656c6c6f");
        assert_eq!(info.encoding_type, EncodingType::Hex);
        assert_eq!(info.confidence, DetectionConfidence::Medium);
    }

    #[test]
    fn test_detect_base64_default() {
        let info = detect_encoding("SGVsbG8=");
        assert_eq!(info.encoding_type, EncodingType::Base64);
    }
}
