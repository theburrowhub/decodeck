//! Base64 decoding module

pub mod variants;

use crate::error::DecodeckError;
use base64::Engine;

/// Base64 encoding variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Base64Variant {
    /// Standard Base64 (RFC 4648): A-Za-z0-9+/
    Standard,
    /// URL-safe Base64 (RFC 4648 §5): A-Za-z0-9-_
    UrlSafe,
}

/// Represents parsed and normalized Base64 data
#[derive(Debug, Clone)]
pub struct EncodedData {
    /// Clean Base64 string (whitespace removed)
    pub data: String,
    /// Detected encoding variant
    pub variant: Base64Variant,
    /// Whether original had padding
    pub has_padding: bool,
    /// Original length before cleaning
    pub original_length: usize,
}

impl EncodedData {
    /// Parse and normalize Base64 input
    pub fn parse(input: &str) -> Result<Self, DecodeckError> {
        let original_length = input.len();

        // Strip whitespace
        let cleaned: String = input.chars().filter(|c| !c.is_whitespace()).collect();

        if cleaned.is_empty() {
            return Err(DecodeckError::NoInput);
        }

        // Detect variant
        let variant = variants::detect_variant(&cleaned);

        // Check for padding
        let has_padding = cleaned.ends_with('=');

        // Add padding if needed
        let data = add_padding(&cleaned);

        // Validate characters
        validate_base64(&data, variant)?;

        Ok(Self {
            data,
            variant,
            has_padding,
            original_length,
        })
    }

    /// Decode the Base64 data to bytes
    pub fn decode(&self) -> Result<Vec<u8>, DecodeckError> {
        let result = match self.variant {
            Base64Variant::Standard => {
                base64::engine::general_purpose::STANDARD.decode(&self.data)
            }
            Base64Variant::UrlSafe => {
                base64::engine::general_purpose::URL_SAFE.decode(&self.data)
            }
        };

        result.map_err(|e| DecodeckError::DecodeFailed {
            message: e.to_string(),
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

/// Validate Base64 characters
fn validate_base64(input: &str, variant: Base64Variant) -> Result<(), DecodeckError> {
    let valid_chars: &str = match variant {
        Base64Variant::Standard => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=",
        Base64Variant::UrlSafe => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_=",
    };

    for (pos, c) in input.chars().enumerate() {
        if !valid_chars.contains(c) {
            return Err(DecodeckError::InvalidBase64 {
                message: format!("Invalid character '{}'", c),
                position: pos,
            });
        }
    }

    Ok(())
}
