//! Input source handling module

pub mod sources;

use crate::error::DecodeckError;
use std::path::PathBuf;

/// Type of input source
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceType {
    /// Data passed as CLI argument
    Arg,
    /// Data read from file
    File,
    /// Data read from stdin
    Stdin,
}

/// Represents the input source for Base64 data
#[derive(Debug, Clone)]
pub struct InputSource {
    /// Type of source
    pub source_type: SourceType,
    /// Raw input data
    pub raw_data: Vec<u8>,
    /// File path (if source_type == File)
    pub path: Option<PathBuf>,
    /// Size in bytes
    pub size_bytes: usize,
}

impl InputSource {
    /// Create InputSource from raw data
    pub fn new(source_type: SourceType, data: Vec<u8>, path: Option<PathBuf>) -> Self {
        let size_bytes = data.len();
        Self {
            source_type,
            raw_data: data,
            path,
            size_bytes,
        }
    }

    /// Validate that input size is within limit
    pub fn validate_size(&self, max_size: &str) -> Result<(), DecodeckError> {
        let limit_bytes = parse_size(max_size)?;
        if self.size_bytes > limit_bytes {
            return Err(DecodeckError::SizeExceeded {
                actual: format_size_short(self.size_bytes),
                limit: max_size.to_string(),
            });
        }
        Ok(())
    }
}

/// Parse size string like "100MB", "1GB", "500KB" to bytes
pub fn parse_size(size_str: &str) -> Result<usize, DecodeckError> {
    let size_str = size_str.trim().to_uppercase();

    // Extract numeric part and unit
    let (num_str, unit) = if size_str.ends_with("GB") {
        (&size_str[..size_str.len() - 2], "GB")
    } else if size_str.ends_with("MB") {
        (&size_str[..size_str.len() - 2], "MB")
    } else if size_str.ends_with("KB") {
        (&size_str[..size_str.len() - 2], "KB")
    } else if size_str.ends_with('B') {
        (&size_str[..size_str.len() - 1], "B")
    } else {
        // Assume bytes if no unit
        (size_str.as_str(), "B")
    };

    let num: f64 = num_str.trim().parse().map_err(|_| DecodeckError::SystemError {
        message: format!("Invalid size format: {}", size_str),
    })?;

    let multiplier: usize = match unit {
        "GB" => 1024 * 1024 * 1024,
        "MB" => 1024 * 1024,
        "KB" => 1024,
        _ => 1,
    };

    Ok((num * multiplier as f64) as usize)
}

/// Format size in short form
fn format_size_short(bytes: usize) -> String {
    const KB: usize = 1024;
    const MB: usize = KB * 1024;
    const GB: usize = MB * 1024;

    if bytes >= GB {
        format!("{:.1}GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1}MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1}KB", bytes as f64 / KB as f64)
    } else {
        format!("{}B", bytes)
    }
}
