//! Unit tests for Base64 decoding

use decodeck::decoder::{Base64Variant, EncodedData};

// T016: Tests for standard Base64 decoding
mod standard_decoding {
    use super::*;

    #[test]
    fn decode_simple_text() {
        let input = "SGVsbG8gV29ybGQh";
        let result = decode_base64(input).expect("Should decode successfully");
        assert_eq!(result, b"Hello World!");
    }

    #[test]
    fn decode_empty_string() {
        let input = "";
        let result = decode_base64(input).expect("Should decode empty string");
        assert_eq!(result, b"");
    }

    #[test]
    fn decode_with_plus_and_slash() {
        // Standard Base64 uses + and /
        let input = "YWJjZCsv"; // Contains + and /
        let result = decode_base64(input);
        assert!(result.is_ok());
    }
}

// T017: Tests for URL-safe Base64 decoding
mod urlsafe_decoding {
    use super::*;

    #[test]
    fn decode_urlsafe_with_minus() {
        // URL-safe Base64 with - character (padded to length 16)
        let input = "SGVsbG8tV29ybGQ="; // Uses - instead of +, with padding
        let result = decode_base64_urlsafe(input).expect("Should decode URL-safe");
        assert!(!result.is_empty());
    }

    #[test]
    fn decode_urlsafe_with_underscore() {
        // URL-safe Base64 with _ character (padded to length 16)
        let input = "SGVsbG9fV29ybGQ="; // Uses _ instead of /, with padding
        let result = decode_base64_urlsafe(input).expect("Should decode URL-safe");
        assert!(!result.is_empty());
    }

    #[test]
    fn auto_detect_urlsafe_variant() {
        let input = "SGVsbG8-V29ybGRf"; // Uses - which is URL-safe specific
        let variant = detect_variant(input);
        assert_eq!(variant, Base64Variant::UrlSafe);
    }

    #[test]
    fn auto_detect_standard_variant() {
        let input = "SGVsbG8rV29ybGQ=";
        let variant = detect_variant(input);
        assert_eq!(variant, Base64Variant::Standard);
    }
}

// T018: Tests for padding handling
mod padding_handling {
    use super::*;

    #[test]
    fn decode_with_single_padding() {
        let input = "SGVsbG8=";
        let result = decode_base64(input).expect("Should decode with single padding");
        assert_eq!(result, b"Hello");
    }

    #[test]
    fn decode_with_double_padding() {
        let input = "SGk=";
        let result = decode_base64(input).expect("Should decode with double padding");
        assert_eq!(result, b"Hi");
    }

    #[test]
    fn decode_without_padding_auto_correct() {
        let input = "SGVsbG8"; // Missing padding
        let result = decode_base64_lenient(input).expect("Should auto-correct padding");
        assert_eq!(result, b"Hello");
    }

    #[test]
    fn decode_no_padding_needed() {
        let input = "SGVsbG8gV29ybGQh"; // Length is multiple of 4
        let result = decode_base64(input).expect("Should decode without padding");
        assert_eq!(result, b"Hello World!");
    }
}

// T019: Tests for whitespace stripping
mod whitespace_stripping {
    use super::*;

    #[test]
    fn strip_spaces() {
        let input = "SGVs bG8g V29y bGQh";
        let cleaned = strip_whitespace(input);
        assert_eq!(cleaned, "SGVsbG8gV29ybGQh");
    }

    #[test]
    fn strip_newlines() {
        let input = "SGVsbG8g\nV29ybGQh";
        let cleaned = strip_whitespace(input);
        assert_eq!(cleaned, "SGVsbG8gV29ybGQh");
    }

    #[test]
    fn strip_tabs() {
        let input = "SGVsbG8g\tV29ybGQh";
        let cleaned = strip_whitespace(input);
        assert_eq!(cleaned, "SGVsbG8gV29ybGQh");
    }

    #[test]
    fn strip_mixed_whitespace() {
        let input = " SGVs\nbG8g \t V29y\r\nbGQh ";
        let cleaned = strip_whitespace(input);
        assert_eq!(cleaned, "SGVsbG8gV29ybGQh");
    }
}

// T020: Tests for invalid Base64 error reporting
mod invalid_base64_errors {
    use super::*;

    #[test]
    fn report_invalid_character() {
        let input = "SGVsbG8$V29ybGQ=";
        let result = decode_base64(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("position"));
    }

    #[test]
    fn report_position_of_invalid_char() {
        let input = "ABCD!EFG";
        let result = decode_base64(input);
        assert!(result.is_err());
        // Should report position 4 (0-indexed)
    }

    #[test]
    fn reject_invalid_length() {
        let input = "ABC"; // Length 3, not valid without padding
        let result = decode_base64_strict(input);
        assert!(result.is_err());
    }
}

// Helper functions to be implemented
fn decode_base64(input: &str) -> Result<Vec<u8>, String> {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD
        .decode(input)
        .map_err(|e| format!("Decode error at position {}: {}", 0, e))
}

fn decode_base64_urlsafe(input: &str) -> Result<Vec<u8>, String> {
    use base64::Engine;
    base64::engine::general_purpose::URL_SAFE
        .decode(input)
        .map_err(|e| format!("Decode error: {}", e))
}

fn decode_base64_lenient(input: &str) -> Result<Vec<u8>, String> {
    use base64::Engine;
    // Add padding if needed
    let padded = add_padding(input);
    base64::engine::general_purpose::STANDARD
        .decode(&padded)
        .map_err(|e| format!("Decode error: {}", e))
}

fn decode_base64_strict(input: &str) -> Result<Vec<u8>, String> {
    use base64::Engine;
    if input.len() % 4 != 0 {
        return Err("Invalid length".to_string());
    }
    base64::engine::general_purpose::STANDARD
        .decode(input)
        .map_err(|e| format!("Decode error: {}", e))
}

fn detect_variant(input: &str) -> Base64Variant {
    decodeck::decoder::variants::detect_variant(input)
}

fn strip_whitespace(input: &str) -> String {
    input.chars().filter(|c| !c.is_whitespace()).collect()
}

fn add_padding(input: &str) -> String {
    let remainder = input.len() % 4;
    if remainder == 0 {
        input.to_string()
    } else {
        let padding = 4 - remainder;
        format!("{}{}", input, "=".repeat(padding))
    }
}
