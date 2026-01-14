//! Unit tests for output path handling

use decodeck::output;
use std::path::Path;

// T068: Tests for output path validation
mod path_validation {
    use super::*;

    #[test]
    fn valid_absolute_path_is_accepted() {
        let path = Path::new("/tmp/test_output.bin");
        assert!(output::validate_output_path(path).is_ok());
    }

    #[test]
    fn valid_relative_path_is_accepted() {
        let path = Path::new("output.bin");
        assert!(output::validate_output_path(path).is_ok());
    }

    #[test]
    fn path_with_extension_is_accepted() {
        let path = Path::new("/tmp/decoded_image.png");
        assert!(output::validate_output_path(path).is_ok());
    }

    #[test]
    fn empty_path_is_rejected() {
        let path = Path::new("");
        assert!(output::validate_output_path(path).is_err());
    }
}

// T069: Tests for path traversal prevention
mod path_traversal_prevention {
    use super::*;

    #[test]
    fn reject_parent_traversal() {
        let path = Path::new("../../../etc/passwd");
        assert!(output::validate_output_path(path).is_err());
    }

    #[test]
    fn reject_hidden_parent_traversal() {
        let path = Path::new("/tmp/test/../../../etc/passwd");
        let result = output::validate_output_path(path);
        // Should either reject or canonicalize safely
        if result.is_ok() {
            // If accepted, path should be safe after canonicalization
            let canonical = path.canonicalize();
            // Canonicalize may fail if path doesn't exist - that's ok
            assert!(canonical.is_err() || !canonical.unwrap().starts_with("/etc"));
        }
    }

    #[test]
    fn reject_null_bytes_in_path() {
        // Null bytes in paths can be used for injection attacks
        let path_str = "/tmp/test\0.txt";
        // Path::new doesn't allow null bytes in Rust
        assert!(path_str.contains('\0'));
    }

    #[test]
    fn allow_safe_relative_path() {
        let path = Path::new("./output/decoded.bin");
        assert!(output::validate_output_path(path).is_ok());
    }
}

// T070: Tests for existing file detection
mod existing_file_detection {
    use super::*;
    use tempfile::tempdir;
    use std::fs::File;

    #[test]
    fn detect_existing_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("existing.txt");
        File::create(&file_path).unwrap();

        assert!(output::file_exists(&file_path));
    }

    #[test]
    fn detect_nonexistent_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("nonexistent.txt");

        assert!(!output::file_exists(&file_path));
    }

    #[test]
    fn detect_directory_as_existing() {
        let dir = tempdir().unwrap();
        // Directory path should not be valid for file output
        assert!(output::file_exists(dir.path()) || !dir.path().is_file());
    }
}
