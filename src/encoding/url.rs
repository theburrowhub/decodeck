//! URL percent-encoding decoder

use super::Decoder;
use crate::error::DecodeckError;
use percent_encoding::percent_decode_str;

/// URL decoder implementation
pub struct UrlDecoder;

impl Decoder for UrlDecoder {
    fn decode(&self, input: &str) -> Result<Vec<u8>, DecodeckError> {
        if input.is_empty() {
            return Err(DecodeckError::NoInput);
        }

        // Replace + with space (form data encoding)
        let with_spaces = input.replace('+', " ");

        // Decode percent-encoded sequences
        percent_decode_str(&with_spaces)
            .decode_utf8()
            .map(|s| s.into_owned().into_bytes())
            .map_err(|e| DecodeckError::DecodeFailed {
                message: format!("Invalid URL encoding: {}", e),
            })
    }

    fn name(&self) -> &'static str {
        "url"
    }

    fn can_decode(&self, input: &str) -> bool {
        // Check if input contains percent-encoded sequences
        input.contains('%')
            && input
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '%')
                .all(|(i, _)| {
                    // Check if followed by two hex digits
                    input
                        .chars()
                        .skip(i + 1)
                        .take(2)
                        .all(|c| c.is_ascii_hexdigit())
                })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_percent() {
        let decoder = UrlDecoder;
        let result = decoder.decode("Hello%20World%21").unwrap();
        assert_eq!(result, b"Hello World!");
    }

    #[test]
    fn test_decode_plus_as_space() {
        let decoder = UrlDecoder;
        let result = decoder.decode("Hello+World").unwrap();
        assert_eq!(result, b"Hello World");
    }

    #[test]
    fn test_decode_utf8() {
        let decoder = UrlDecoder;
        let result = decoder.decode("%C3%A1%C3%A9").unwrap();
        assert_eq!(String::from_utf8(result).unwrap(), "áé");
    }

    #[test]
    fn test_can_decode() {
        let decoder = UrlDecoder;
        assert!(decoder.can_decode("Hello%20World"));
        assert!(decoder.can_decode("%2F%2F"));
        assert!(!decoder.can_decode("Hello World")); // no percent encoding
    }
}
