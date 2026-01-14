//! Text output formatter

use super::DecodeResult;
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

    if let Some(ref encoding) = result.encoding {
        let variant = match encoding.variant {
            crate::decoder::Base64Variant::Standard => "Standard",
            crate::decoder::Base64Variant::UrlSafe => "URL-safe",
        };
        let padding = if encoding.has_padding {
            "with padding"
        } else {
            "without padding (added)"
        };
        writeln!(writer, "Encoding: {} Base64 ({})", variant, padding)?;
    }

    if !result.warnings.is_empty() {
        writeln!(writer)?;
        for warning in &result.warnings {
            writeln!(writer, "Warning: {}", warning)?;
        }
    }

    Ok(())
}
