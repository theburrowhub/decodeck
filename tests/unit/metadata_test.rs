//! Unit tests for metadata detection

use decodeck::metadata::{ContentCategory, ContentMetadata};
use decodeck::metadata::magic;

// T037: Tests for MIME detection
mod mime_detection {
    use super::*;

    #[test]
    fn detect_png_from_magic_bytes() {
        // PNG magic bytes: 89 50 4E 47 0D 0A 1A 0A
        let png_data = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00];
        let metadata = magic::detect(&png_data);
        assert_eq!(metadata.mime_type, "image/png");
    }

    #[test]
    fn detect_pdf_from_magic_bytes() {
        // PDF magic bytes: %PDF
        let pdf_data = b"%PDF-1.4\n%...";
        let metadata = magic::detect(pdf_data);
        assert_eq!(metadata.mime_type, "application/pdf");
    }

    #[test]
    fn detect_mp3_from_magic_bytes() {
        // MP3 with ID3 tag: ID3
        let mp3_data = [0x49, 0x44, 0x33, 0x04, 0x00, 0x00, 0x00, 0x00];
        let metadata = magic::detect(&mp3_data);
        assert!(metadata.mime_type.contains("audio") || metadata.mime_type == "application/octet-stream");
    }

    #[test]
    fn detect_jpeg_from_magic_bytes() {
        // JPEG magic bytes: FF D8 FF
        let jpeg_data = [0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46];
        let metadata = magic::detect(&jpeg_data);
        assert_eq!(metadata.mime_type, "image/jpeg");
    }

    #[test]
    fn detect_unknown_returns_octet_stream() {
        let random_data = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
        let metadata = magic::detect(&random_data);
        assert_eq!(metadata.mime_type, "application/octet-stream");
    }
}

// T038: Tests for extension mapping
mod extension_mapping {
    use super::*;

    #[test]
    fn png_has_png_extension() {
        let png_data = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00];
        let metadata = magic::detect(&png_data);
        assert_eq!(metadata.extension, ".png");
    }

    #[test]
    fn pdf_has_pdf_extension() {
        let pdf_data = b"%PDF-1.4\n%...";
        let metadata = magic::detect(pdf_data);
        assert_eq!(metadata.extension, ".pdf");
    }

    #[test]
    fn jpeg_has_jpg_extension() {
        let jpeg_data = [0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46];
        let metadata = magic::detect(&jpeg_data);
        assert!(metadata.extension == ".jpg" || metadata.extension == ".jpeg");
    }

    #[test]
    fn unknown_has_bin_extension() {
        let random_data = [0x00, 0x01, 0x02, 0x03];
        let metadata = magic::detect(&random_data);
        assert_eq!(metadata.extension, ".bin");
    }
}

// T039: Tests for size formatting
mod size_formatting {
    use decodeck::output::format_size;

    #[test]
    fn format_bytes() {
        assert_eq!(format_size(0), "0 bytes");
        assert_eq!(format_size(1), "1 bytes");
        assert_eq!(format_size(512), "512 bytes");
        assert_eq!(format_size(1023), "1023 bytes");
    }

    #[test]
    fn format_kilobytes() {
        assert_eq!(format_size(1024), "1.0 KB");
        assert_eq!(format_size(1536), "1.5 KB");
        assert_eq!(format_size(10240), "10.0 KB");
    }

    #[test]
    fn format_megabytes() {
        assert_eq!(format_size(1048576), "1.0 MB");
        assert_eq!(format_size(1572864), "1.5 MB");
        assert_eq!(format_size(10485760), "10.0 MB");
    }

    #[test]
    fn format_gigabytes() {
        assert_eq!(format_size(1073741824), "1.0 GB");
        assert_eq!(format_size(1610612736), "1.5 GB");
    }
}

// T040: Tests for ContentCategory classification
mod category_classification {
    use super::*;

    #[test]
    fn png_is_image_category() {
        let png_data = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00];
        let metadata = magic::detect(&png_data);
        assert_eq!(metadata.category, ContentCategory::Image);
    }

    #[test]
    fn pdf_is_document_category() {
        let pdf_data = b"%PDF-1.4\n%...";
        let metadata = magic::detect(pdf_data);
        assert_eq!(metadata.category, ContentCategory::Document);
    }

    #[test]
    fn jpeg_is_image_category() {
        let jpeg_data = [0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46];
        let metadata = magic::detect(&jpeg_data);
        assert_eq!(metadata.category, ContentCategory::Image);
    }

    #[test]
    fn unknown_is_other_category() {
        let random_data = [0x00, 0x01, 0x02, 0x03];
        let metadata = magic::detect(&random_data);
        assert_eq!(metadata.category, ContentCategory::Other);
    }

    #[test]
    fn image_is_viewable() {
        let png_data = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00];
        let metadata = magic::detect(&png_data);
        assert!(metadata.is_viewable);
        assert!(!metadata.is_playable);
    }

    #[test]
    fn pdf_is_viewable() {
        let pdf_data = b"%PDF-1.4\n%...";
        let metadata = magic::detect(pdf_data);
        assert!(metadata.is_viewable);
        assert!(!metadata.is_playable);
    }
}
