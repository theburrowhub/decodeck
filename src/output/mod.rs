//! Output handling module

pub mod json;
pub mod text;

use crate::decoder::EncodedData;
use crate::encoding::EncodingInfo;
use crate::error::DecodeckError;
use crate::metadata::ContentMetadata;
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Represents the output file
#[derive(Debug, Clone, Serialize)]
pub struct OutputFile {
    /// Full path to the file
    pub path: PathBuf,
    /// Whether this is a temporary file
    pub is_temporary: bool,
    /// Size in bytes
    pub size_bytes: usize,
    /// Formatted size (e.g., "1.2 MB")
    pub size_formatted: String,
    /// Creation timestamp
    #[serde(skip)]
    pub created_at: Option<SystemTime>,
}

/// Complete decode operation result
#[derive(Debug, Clone, Serialize)]
pub struct DecodeResult {
    /// Whether operation succeeded
    pub success: bool,
    /// Output file information
    pub output: OutputFile,
    /// Content metadata
    pub metadata: ContentMetadata,
    /// Legacy Base64 encoding information (for backwards compatibility)
    #[serde(skip)]
    pub encoding: Option<EncodedData>,
    /// Multi-encoding type information
    pub encoding_info: EncodingInfo,
    /// Processing duration in milliseconds
    pub duration_ms: u64,
    /// Any warnings generated
    pub warnings: Vec<String>,
}

/// Format bytes as human-readable string
pub fn format_size(bytes: usize) -> String {
    const KB: usize = 1024;
    const MB: usize = KB * 1024;
    const GB: usize = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

/// Validate output path for safety
pub fn validate_output_path(path: &Path) -> Result<(), DecodeckError> {
    // Reject empty paths
    if path.as_os_str().is_empty() {
        return Err(DecodeckError::InvalidOutputPath {
            path: String::new(),
            reason: "Path cannot be empty".to_string(),
        });
    }

    // Check for path traversal attempts
    let path_str = path.to_string_lossy();
    if path_str.contains("..") {
        // More sophisticated check: actually resolve components
        let mut has_traversal = false;
        for component in path.components() {
            if let std::path::Component::ParentDir = component {
                has_traversal = true;
                break;
            }
        }
        if has_traversal {
            return Err(DecodeckError::InvalidOutputPath {
                path: path_str.to_string(),
                reason: "Path traversal not allowed".to_string(),
            });
        }
    }

    Ok(())
}

/// Check if a file exists at the given path
pub fn file_exists(path: &Path) -> bool {
    path.exists() && path.is_file()
}
