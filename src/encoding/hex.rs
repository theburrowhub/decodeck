//! Hexadecimal encoding decoder

use super::Decoder;
use crate::error::DecodeckError;

/// Hex decoder implementation
pub struct HexDecoder;

impl Decoder for HexDecoder {
    fn decode(&self, input: &str) -> Result<Vec<u8>, DecodeckError> {
        // Strip whitespace and 0x prefix
        let cleaned = input
            .trim()
            .strip_prefix("0x")
            .or_else(|| input.trim().strip_prefix("0X"))
            .unwrap_or(input.trim());

        // Remove all whitespace
        let hex_str: String = cleaned.chars().filter(|c| !c.is_whitespace()).collect();

        if hex_str.is_empty() {
            return Err(DecodeckError::NoInput);
        }

        // Validate and decode
        data_encoding::HEXLOWER_PERMISSIVE
            .decode(hex_str.as_bytes())
            .map_err(|e| DecodeckError::DecodeFailed {
                message: format!("Invalid hex: {}", e),
            })
    }

    fn name(&self) -> &'static str {
        "hex"
    }

    fn can_decode(&self, input: &str) -> bool {
        let cleaned = input
            .trim()
            .strip_prefix("0x")
            .or_else(|| input.trim().strip_prefix("0X"))
            .unwrap_or(input.trim());

        let hex_str: String = cleaned.chars().filter(|c| !c.is_whitespace()).collect();

        if hex_str.is_empty() {
            return false;
        }

        // Must be even length and only hex chars
        hex_str.len() % 2 == 0 && hex_str.chars().all(|c| c.is_ascii_hexdigit())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_simple() {
        let decoder = HexDecoder;
        let result = decoder.decode("48656c6c6f").unwrap();
        assert_eq!(result, b"Hello");
    }

    #[test]
    fn test_decode_with_0x_prefix() {
        let decoder = HexDecoder;
        let result = decoder.decode("0x48656c6c6f").unwrap();
        assert_eq!(result, b"Hello");
    }

    #[test]
    fn test_decode_with_spaces() {
        let decoder = HexDecoder;
        let result = decoder.decode("48 65 6c 6c 6f").unwrap();
        assert_eq!(result, b"Hello");
    }

    #[test]
    fn test_decode_uppercase() {
        let decoder = HexDecoder;
        let result = decoder.decode("48656C6C6F").unwrap();
        assert_eq!(result, b"Hello");
    }

    #[test]
    fn test_can_decode() {
        let decoder = HexDecoder;
        assert!(decoder.can_decode("48656c6c6f"));
        assert!(decoder.can_decode("0x48656c6c6f"));
        assert!(!decoder.can_decode("48656c6c6")); // odd length
        assert!(!decoder.can_decode("ghijkl")); // invalid chars
    }
}
