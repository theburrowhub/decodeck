//! Text output formatter

use super::DecodeResult;
use crate::encoding::{DetectionConfidence, EncodingType};
use std::io::Write;

/// Format decode result as human-readable text
pub fn format(result: &DecodeResult, writer: &mut impl Write) -> std::io::Result<()> {
    writeln!(writer, "Decoded: {}", result.output.path.display())?;
    writeln!(
        writer,
        "Size: {} ({} bytes)",
        result.output.size_formatted, result.output.size_bytes
    )?;
    writeln!(writer, "Type: {}", result.metadata.mime_type)?;
    writeln!(writer, "Extension: {}", result.metadata.extension)?;

    // Show encoding information
    let encoding_info = &result.encoding_info;
    let detection_str = if encoding_info.detected {
        match encoding_info.confidence {
            DetectionConfidence::High => " (auto-detected, high confidence)",
            DetectionConfidence::Medium => " (auto-detected, medium confidence)",
            DetectionConfidence::Low => " (auto-detected, low confidence)",
            DetectionConfidence::Explicit => "",
        }
    } else {
        " (specified)"
    };

    // For Base64, show additional variant info if available
    if encoding_info.encoding_type == EncodingType::Base64 {
        if let Some(ref enc) = result.encoding {
            let variant = match enc.variant {
                crate::decoder::Base64Variant::Standard => "Standard",
                crate::decoder::Base64Variant::UrlSafe => "URL-safe",
            };
            let padding = if enc.has_padding {
                "with padding"
            } else {
                "without padding (added)"
            };
            writeln!(
                writer,
                "Encoding: {} Base64 ({}){}",
                variant, padding, detection_str
            )?;
        } else {
            writeln!(writer, "Encoding: Base64{}", detection_str)?;
        }
    } else {
        writeln!(
            writer,
            "Encoding: {}{}",
            encoding_info.encoding_type, detection_str
        )?;
    }

    if !result.warnings.is_empty() {
        writeln!(writer)?;
        for warning in &result.warnings {
            writeln!(writer, "Warning: {}", warning)?;
        }
    }

    Ok(())
}
