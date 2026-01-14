//! Content metadata detection module

pub mod magic;

use serde::Serialize;

/// Content category classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ContentCategory {
    Image,
    Document,
    Audio,
    Video,
    Archive,
    Other,
}

/// Metadata about decoded content
#[derive(Debug, Clone, Serialize)]
pub struct ContentMetadata {
    /// MIME type (e.g., "image/png")
    pub mime_type: String,
    /// Suggested file extension (e.g., ".png")
    pub extension: String,
    /// Content category
    pub category: ContentCategory,
    /// Whether content can be viewed (images, PDFs, etc.)
    pub is_viewable: bool,
    /// Whether content can be played (audio, video)
    pub is_playable: bool,
    /// First 8 bytes for debugging
    #[serde(skip)]
    pub magic_bytes: Option<[u8; 8]>,
}

impl ContentMetadata {
    /// Create metadata with unknown type
    pub fn unknown() -> Self {
        Self {
            mime_type: "application/octet-stream".to_string(),
            extension: ".bin".to_string(),
            category: ContentCategory::Other,
            is_viewable: false,
            is_playable: false,
            magic_bytes: None,
        }
    }
}
