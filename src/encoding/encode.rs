//! Encoding functions for all supported formats

use crate::encoding::EncodingType;
use crate::error::DecodeckError;
use base64::Engine;

/// Encode bytes to string using the specified encoding
pub fn encode(data: &[u8], encoding: EncodingType) -> Result<String, DecodeckError> {
    match encoding {
        EncodingType::Base64 => Ok(encode_base64(data)),
        EncodingType::Hex => Ok(encode_hex(data)),
        EncodingType::Base32 => Ok(encode_base32(data)),
        EncodingType::Url => encode_url(data),
        EncodingType::Base85 => Ok(encode_base85(data)),
    }
}

/// Encode to Base64
fn encode_base64(data: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(data)
}

/// Encode to hexadecimal
fn encode_hex(data: &[u8]) -> String {
    data_encoding::HEXLOWER.encode(data)
}

/// Encode to Base32
fn encode_base32(data: &[u8]) -> String {
    data_encoding::BASE32.encode(data)
}

/// Encode to URL percent-encoding
fn encode_url(data: &[u8]) -> Result<String, DecodeckError> {
    let s = std::str::from_utf8(data).map_err(|e| DecodeckError::DecodeFailed {
        message: format!("URL encoding requires valid UTF-8: {}", e),
    })?;
    Ok(percent_encoding::utf8_percent_encode(s, percent_encoding::NON_ALPHANUMERIC).to_string())
}

/// Encode to Ascii85
fn encode_base85(data: &[u8]) -> String {
    // ascii85::encode already adds delimiters
    ascii85::encode(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_base64() {
        let result = encode(b"Hello", EncodingType::Base64).unwrap();
        assert_eq!(result, "SGVsbG8=");
    }

    #[test]
    fn test_encode_hex() {
        let result = encode(b"Hello", EncodingType::Hex).unwrap();
        assert_eq!(result, "48656c6c6f");
    }

    #[test]
    fn test_encode_base32() {
        let result = encode(b"Hello", EncodingType::Base32).unwrap();
        assert_eq!(result, "JBSWY3DP");
    }

    #[test]
    fn test_encode_url() {
        let result = encode(b"Hello World!", EncodingType::Url).unwrap();
        assert_eq!(result, "Hello%20World%21");
    }

    #[test]
    fn test_encode_base85() {
        let result = encode(b"test", EncodingType::Base85).unwrap();
        // ascii85 crate adds delimiters automatically
        assert!(result.starts_with("<~") && result.ends_with("~>"));
        assert!(result.contains("FCfN8"));
    }
}
