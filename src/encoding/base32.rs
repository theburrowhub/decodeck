//! Base32 encoding decoder

use super::Decoder;
use crate::error::DecodeckError;

/// Base32 decoder implementation
pub struct Base32Decoder;

impl Decoder for Base32Decoder {
    fn decode(&self, input: &str) -> Result<Vec<u8>, DecodeckError> {
        // Strip whitespace and convert to uppercase
        let cleaned: String = input
            .chars()
            .filter(|c| !c.is_whitespace())
            .flat_map(|c| c.to_uppercase())
            .collect();

        if cleaned.is_empty() {
            return Err(DecodeckError::NoInput);
        }

        // Add padding if needed (Base32 needs multiples of 8)
        let padded = add_base32_padding(&cleaned);

        data_encoding::BASE32
            .decode(padded.as_bytes())
            .map_err(|e| DecodeckError::DecodeFailed {
                message: format!("Invalid Base32: {}", e),
            })
    }

    fn name(&self) -> &'static str {
        "base32"
    }

    fn can_decode(&self, input: &str) -> bool {
        let cleaned: String = input
            .chars()
            .filter(|c| !c.is_whitespace())
            .flat_map(|c| c.to_uppercase())
            .collect();

        if cleaned.is_empty() {
            return false;
        }

        // Base32 uses A-Z and 2-7 (and = for padding)
        cleaned
            .chars()
            .all(|c| matches!(c, 'A'..='Z' | '2'..='7' | '='))
    }
}

/// Add padding to Base32 string if needed
fn add_base32_padding(input: &str) -> String {
    let remainder = input.len() % 8;
    if remainder == 0 {
        input.to_string()
    } else {
        let padding = 8 - remainder;
        format!("{}{}", input, "=".repeat(padding))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_simple() {
        let decoder = Base32Decoder;
        let result = decoder.decode("JBSWY3DPEBLW64TMMQ======").unwrap();
        assert_eq!(result, b"Hello World");
    }

    #[test]
    fn test_decode_lowercase() {
        let decoder = Base32Decoder;
        let result = decoder.decode("jbswy3dpeblw64tmmq").unwrap();
        assert_eq!(result, b"Hello World");
    }

    #[test]
    fn test_decode_without_padding() {
        let decoder = Base32Decoder;
        let result = decoder.decode("JBSWY3DPEBLW64TMMQ").unwrap();
        assert_eq!(result, b"Hello World");
    }

    #[test]
    fn test_can_decode() {
        let decoder = Base32Decoder;
        assert!(decoder.can_decode("JBSWY3DP"));
        assert!(!decoder.can_decode("jbswy3d8")); // 8 is invalid
        assert!(!decoder.can_decode("")); // empty
    }
}
