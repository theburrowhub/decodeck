//! JSON output formatter

use super::DecodeResult;
use serde::Serialize;
use std::io::Write;

/// JSON output structure
#[derive(Serialize)]
pub struct JsonOutput<'a> {
    pub success: bool,
    pub output: OutputInfo<'a>,
    pub metadata: MetadataInfo<'a>,
    pub encoding: EncodingInfo,
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
pub struct EncodingInfo {
    pub variant: String,
    pub had_padding: bool,
}

/// Format decode result as JSON
pub fn format(result: &DecodeResult, writer: &mut impl Write) -> std::io::Result<()> {
    let category = format!("{:?}", result.metadata.category).to_lowercase();

    let encoding = if let Some(ref enc) = result.encoding {
        EncodingInfo {
            variant: match enc.variant {
                crate::decoder::Base64Variant::Standard => "standard".to_string(),
                crate::decoder::Base64Variant::UrlSafe => "url-safe".to_string(),
            },
            had_padding: enc.has_padding,
        }
    } else {
        EncodingInfo {
            variant: "unknown".to_string(),
            had_padding: false,
        }
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
