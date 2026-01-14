//! Ascii85 (Base85) encoding decoder

use super::Decoder;
use crate::error::DecodeckError;

/// Base85/Ascii85 decoder implementation
pub struct Base85Decoder;

impl Decoder for Base85Decoder {
    fn decode(&self, input: &str) -> Result<Vec<u8>, DecodeckError> {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return Err(DecodeckError::NoInput);
        }

        // Strip <~ and ~> delimiters if present
        let data = if trimmed.starts_with("<~") && trimmed.ends_with("~>") {
            &trimmed[2..trimmed.len() - 2]
        } else {
            trimmed
        };

        ascii85::decode(data).map_err(|e| DecodeckError::DecodeFailed {
            message: format!("Invalid Ascii85: {:?}", e),
        })
    }

    fn name(&self) -> &'static str {
        "base85"
    }

    fn can_decode(&self, input: &str) -> bool {
        let trimmed = input.trim();
        // Check for Ascii85 delimiters
        trimmed.starts_with("<~") && trimmed.ends_with("~>")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_with_delimiters() {
        let decoder = Base85Decoder;
        // "Hello " (6 chars) in Ascii85
        let result = decoder.decode("<~87cURD]j~>").unwrap();
        assert_eq!(result, b"Hello ");
    }

    #[test]
    fn test_decode_without_delimiters() {
        let decoder = Base85Decoder;
        let result = decoder.decode("87cURD]j").unwrap();
        assert_eq!(result, b"Hello ");
    }

    #[test]
    fn test_decode_test() {
        let decoder = Base85Decoder;
        // "test" (4 chars) in Ascii85
        let result = decoder.decode("FCfN8").unwrap();
        assert_eq!(result, b"test");
    }

    #[test]
    fn test_can_decode() {
        let decoder = Base85Decoder;
        assert!(decoder.can_decode("<~87cURD]j~>"));
        assert!(!decoder.can_decode("87cURD]j")); // no delimiters
    }
}
