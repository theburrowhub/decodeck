//! Multi-encoding support module
//!
//! Provides a unified interface for decoding various encoding formats:
//! Base64, Hex, Base32, URL percent-encoding, and Ascii85.

pub mod base64;
pub mod base32;
pub mod base85;
pub mod detect;
pub mod hex;
pub mod url;

use crate::error::DecodeckError;
use clap::ValueEnum;
use serde::Serialize;

/// Trait for all encoding decoders
pub trait Decoder {
    /// Decode the input string to bytes
    fn decode(&self, input: &str) -> Result<Vec<u8>, DecodeckError>;

    /// Get the name of this encoding
    fn name(&self) -> &'static str;

    /// Check if this decoder can likely decode the input (for auto-detection)
    fn can_decode(&self, input: &str) -> bool;
}

/// Supported encoding types
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EncodingType {
    /// Standard or URL-safe Base64 (RFC 4648)
    Base64,
    /// Hexadecimal encoding
    Hex,
    /// Base32 encoding (RFC 4648)
    Base32,
    /// URL percent-encoding (RFC 3986)
    Url,
    /// Ascii85 encoding (Adobe variant)
    Base85,
}

impl EncodingType {
    /// Get a decoder instance for this encoding type
    pub fn decoder(&self) -> Box<dyn Decoder> {
        match self {
            EncodingType::Base64 => Box::new(base64::Base64Decoder),
            EncodingType::Hex => Box::new(hex::HexDecoder),
            EncodingType::Base32 => Box::new(base32::Base32Decoder),
            EncodingType::Url => Box::new(url::UrlDecoder),
            EncodingType::Base85 => Box::new(base85::Base85Decoder),
        }
    }

    /// Get human-readable name
    pub fn display_name(&self) -> &'static str {
        match self {
            EncodingType::Base64 => "base64",
            EncodingType::Hex => "hex",
            EncodingType::Base32 => "base32",
            EncodingType::Url => "url",
            EncodingType::Base85 => "base85",
        }
    }
}

impl std::fmt::Display for EncodingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Result of encoding detection
#[derive(Debug, Clone, Serialize)]
pub struct EncodingInfo {
    /// The detected or specified encoding type
    #[serde(rename = "type")]
    pub encoding_type: EncodingType,
    /// Whether the encoding was auto-detected
    pub detected: bool,
    /// Confidence level of detection
    pub confidence: DetectionConfidence,
}

/// Confidence level for auto-detection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DetectionConfidence {
    /// Explicit user specification
    Explicit,
    /// High confidence (clear markers like 0x prefix)
    High,
    /// Medium confidence (character analysis)
    Medium,
    /// Low confidence (fallback to default)
    Low,
}

impl EncodingInfo {
    /// Create info for explicitly specified encoding
    pub fn explicit(encoding_type: EncodingType) -> Self {
        Self {
            encoding_type,
            detected: false,
            confidence: DetectionConfidence::Explicit,
        }
    }

    /// Create info for auto-detected encoding
    pub fn detected(encoding_type: EncodingType, confidence: DetectionConfidence) -> Self {
        Self {
            encoding_type,
            detected: true,
            confidence,
        }
    }
}
