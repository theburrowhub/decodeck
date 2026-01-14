//! Magic bytes detection for MIME types

use super::{ContentCategory, ContentMetadata};

/// Detect content metadata from bytes using magic bytes
pub fn detect(data: &[u8]) -> ContentMetadata {
    let magic_bytes = if data.len() >= 8 {
        let mut arr = [0u8; 8];
        arr.copy_from_slice(&data[..8]);
        Some(arr)
    } else {
        None
    };

    // Use infer crate for detection
    if let Some(kind) = infer::get(data) {
        let mime_type = kind.mime_type().to_string();
        let extension = format!(".{}", kind.extension());
        let category = categorize_mime(&mime_type);
        let (is_viewable, is_playable) = viewable_playable(&category);

        ContentMetadata {
            mime_type,
            extension,
            category,
            is_viewable,
            is_playable,
            magic_bytes,
        }
    } else {
        let mut metadata = ContentMetadata::unknown();
        metadata.magic_bytes = magic_bytes;
        metadata
    }
}

fn categorize_mime(mime: &str) -> ContentCategory {
    if mime.starts_with("image/") {
        ContentCategory::Image
    } else if mime.starts_with("video/") {
        ContentCategory::Video
    } else if mime.starts_with("audio/") {
        ContentCategory::Audio
    } else if mime == "application/pdf" || mime.starts_with("text/") {
        ContentCategory::Document
    } else if mime.contains("zip") || mime.contains("tar") || mime.contains("gzip") {
        ContentCategory::Archive
    } else {
        ContentCategory::Other
    }
}

fn viewable_playable(category: &ContentCategory) -> (bool, bool) {
    match category {
        ContentCategory::Image | ContentCategory::Document => (true, false),
        ContentCategory::Video | ContentCategory::Audio => (false, true),
        _ => (false, false),
    }
}
