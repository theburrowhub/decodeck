//! JSON output formatter

use super::DecodeResult;
use crate::encoding::{DetectionConfidence, EncodingType};
use serde::Serialize;
use std::io::Write;

/// JSON output structure
#[derive(Serialize)]
pub struct JsonOutput<'a> {
    pub success: bool,
    pub output: OutputInfo<'a>,
    pub metadata: MetadataInfo<'a>,
    pub encoding: EncodingOutput,
    pub duration_ms: u64,
    pub warnings: &'a [String],
}

#[derive(Serialize)]
pub struct OutputInfo<'a> {
    pub path: &'a str,
    pub is_temporary: bool,
    pub size_bytes: usize,
    pub size_formatted: &'a str,
}

#[derive(Serialize)]
pub struct MetadataInfo<'a> {
    pub mime_type: &'a str,
    pub extension: &'a str,
    pub category: &'a str,
    pub is_viewable: bool,
    pub is_playable: bool,
}

#[derive(Serialize)]
pub struct EncodingOutput {
    #[serde(rename = "type")]
    pub encoding_type: EncodingType,
    pub detected: bool,
    pub confidence: DetectionConfidence,
    /// Base64-specific: variant (standard/url-safe)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    /// Base64-specific: whether input had padding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub had_padding: Option<bool>,
}

/// Format decode result as JSON
pub fn format(result: &DecodeResult, writer: &mut impl Write) -> std::io::Result<()> {
    let category = format!("{:?}", result.metadata.category).to_lowercase();

    // Build encoding output with Base64-specific details if available
    let (variant, had_padding) = if let Some(ref enc) = result.encoding {
        let variant_str = match enc.variant {
            crate::decoder::Base64Variant::Standard => "standard".to_string(),
            crate::decoder::Base64Variant::UrlSafe => "url-safe".to_string(),
        };
        (Some(variant_str), Some(enc.has_padding))
    } else {
        (None, None)
    };

    let encoding = EncodingOutput {
        encoding_type: result.encoding_info.encoding_type,
        detected: result.encoding_info.detected,
        confidence: result.encoding_info.confidence,
        variant,
        had_padding,
    };

    let output = JsonOutput {
        success: result.success,
        output: OutputInfo {
            path: result.output.path.to_str().unwrap_or(""),
            is_temporary: result.output.is_temporary,
            size_bytes: result.output.size_bytes,
            size_formatted: &result.output.size_formatted,
        },
        metadata: MetadataInfo {
            mime_type: &result.metadata.mime_type,
            extension: &result.metadata.extension,
            category: &category,
            is_viewable: result.metadata.is_viewable,
            is_playable: result.metadata.is_playable,
        },
        encoding,
        duration_ms: result.duration_ms,
        warnings: &result.warnings,
    };

    let json = serde_json::to_string_pretty(&output).map_err(std::io::Error::other)?;
    writeln!(writer, "{}", json)
}
