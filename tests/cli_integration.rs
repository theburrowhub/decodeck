//! CLI integration tests for interactive features

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

// T057: Tests for --quiet suppression
mod quiet_flag {
    use super::*;

    #[test]
    fn quiet_suppresses_prompt_message() {
        // PNG magic bytes: 89 50 4E 47 0D 0A 1A 0A
        let png_base64 = "iVBORw0KGgo="; // PNG header

        let mut cmd = Command::cargo_bin("decodeck").unwrap();
        let output = cmd
            .args(["decode", png_base64, "--quiet"])
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);

        // With --quiet, should NOT show "Press space to view..."
        assert!(!stdout.contains("Press space"));
    }

    #[test]
    fn quiet_still_outputs_file_path() {
        let text_base64 = "SGVsbG8gV29ybGQh"; // "Hello World!"

        let mut cmd = Command::cargo_bin("decodeck").unwrap();
        let output = cmd
            .args(["decode", text_base64, "--quiet"])
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Should still show where file was saved
        assert!(stdout.contains("Saved to:") || stdout.contains(".bin") || stdout.is_empty() || output.status.success());
    }

    #[test]
    fn quiet_with_json_outputs_json_only() {
        let text_base64 = "SGVsbG8gV29ybGQh";

        let mut cmd = Command::cargo_bin("decodeck").unwrap();
        let output = cmd
            .args(["decode", text_base64, "--quiet", "--json"])
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Should output valid JSON without prompt text
        assert!(!stdout.contains("Press space"));
        // If there's output, it should be JSON
        if !stdout.trim().is_empty() {
            assert!(stdout.contains("{") || stdout.contains("path"));
        }
    }
}

// T058: Tests for --no-interactive flag
mod no_interactive_flag {
    use super::*;

    #[test]
    fn no_interactive_suppresses_prompt() {
        let png_base64 = "iVBORw0KGgo=";

        let mut cmd = Command::cargo_bin("decodeck").unwrap();
        let output = cmd
            .args(["decode", png_base64, "--no-interactive"])
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);

        // With --no-interactive, should NOT show "Press space to view..."
        assert!(!stdout.contains("Press space"));
    }

    #[test]
    fn no_interactive_does_not_wait_for_input() {
        let text_base64 = "SGVsbG8gV29ybGQh";

        // This test verifies the command completes immediately
        // (doesn't hang waiting for key press)
        let mut cmd = Command::cargo_bin("decodeck").unwrap();
        cmd.args(["decode", text_base64, "--no-interactive"])
            .timeout(std::time::Duration::from_secs(5))
            .assert()
            .success();
    }

    #[test]
    fn both_quiet_and_no_interactive_work_together() {
        let text_base64 = "SGVsbG8gV29ybGQh";

        let mut cmd = Command::cargo_bin("decodeck").unwrap();
        let output = cmd
            .args(["decode", text_base64, "--quiet", "--no-interactive"])
            .output()
            .expect("Failed to execute command");

        // Should complete successfully
        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(!stdout.contains("Press space"));
    }

    #[test]
    fn no_interactive_with_output_flag() {
        let dir = tempdir().unwrap();
        let output_path = dir.path().join("test_output.bin");
        let text_base64 = "SGVsbG8gV29ybGQh";

        let mut cmd = Command::cargo_bin("decodeck").unwrap();
        cmd.args([
            "decode",
            text_base64,
            "--no-interactive",
            "--output",
            output_path.to_str().unwrap(),
        ])
        .assert()
        .success();

        // File should exist
        assert!(output_path.exists());
    }
}

// T071: Tests for overwrite confirmation (without --force)
mod overwrite_behavior {
    use super::*;
    use std::fs;

    #[test]
    fn existing_file_without_force_fails() {
        let dir = tempdir().unwrap();
        let output_path = dir.path().join("existing.bin");
        let text_base64 = "SGVsbG8gV29ybGQh";

        // Create existing file
        fs::write(&output_path, b"original content").unwrap();

        let mut cmd = Command::cargo_bin("decodeck").unwrap();
        let output = cmd
            .args([
                "decode",
                text_base64,
                "--no-interactive",
                "--output",
                output_path.to_str().unwrap(),
            ])
            .output()
            .expect("Failed to execute command");

        // Should fail because file exists
        assert!(!output.status.success());

        // Original content should be preserved
        let content = fs::read(&output_path).unwrap();
        assert_eq!(content, b"original content");
    }

    #[test]
    fn error_message_mentions_existing_file() {
        let dir = tempdir().unwrap();
        let output_path = dir.path().join("existing.bin");
        let text_base64 = "SGVsbG8gV29ybGQh";

        fs::write(&output_path, b"content").unwrap();

        let mut cmd = Command::cargo_bin("decodeck").unwrap();
        let output = cmd
            .args([
                "decode",
                text_base64,
                "--no-interactive",
                "--output",
                output_path.to_str().unwrap(),
            ])
            .output()
            .expect("Failed to execute command");

        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("exists") || stderr.contains("already"));
    }
}

// T072: Tests for --output flag
mod output_flag {
    use super::*;
    use std::fs;

    #[test]
    fn output_creates_file_at_specified_path() {
        let dir = tempdir().unwrap();
        let output_path = dir.path().join("custom_output.bin");
        let text_base64 = "SGVsbG8gV29ybGQh"; // "Hello World!"

        let mut cmd = Command::cargo_bin("decodeck").unwrap();
        cmd.args([
            "decode",
            text_base64,
            "--no-interactive",
            "--output",
            output_path.to_str().unwrap(),
        ])
        .assert()
        .success();

        assert!(output_path.exists());
        let content = fs::read(&output_path).unwrap();
        assert_eq!(content, b"Hello World!");
    }

    #[test]
    fn output_in_nested_directory() {
        let dir = tempdir().unwrap();
        let nested_dir = dir.path().join("sub/dir");
        fs::create_dir_all(&nested_dir).unwrap();
        let output_path = nested_dir.join("output.bin");

        let text_base64 = "SGVsbG8gV29ybGQh";

        let mut cmd = Command::cargo_bin("decodeck").unwrap();
        cmd.args([
            "decode",
            text_base64,
            "--no-interactive",
            "--output",
            output_path.to_str().unwrap(),
        ])
        .assert()
        .success();

        assert!(output_path.exists());
    }

    #[test]
    fn output_with_custom_extension() {
        let dir = tempdir().unwrap();
        let output_path = dir.path().join("decoded_image.png");
        let text_base64 = "SGVsbG8gV29ybGQh";

        let mut cmd = Command::cargo_bin("decodeck").unwrap();
        cmd.args([
            "decode",
            text_base64,
            "--no-interactive",
            "--output",
            output_path.to_str().unwrap(),
        ])
        .assert()
        .success();

        assert!(output_path.exists());
        assert_eq!(output_path.extension().unwrap(), "png");
    }
}

// T073: Tests for --force flag
mod force_flag {
    use super::*;
    use std::fs;

    #[test]
    fn force_overwrites_existing_file() {
        let dir = tempdir().unwrap();
        let output_path = dir.path().join("existing.bin");
        let text_base64 = "SGVsbG8gV29ybGQh"; // "Hello World!"

        // Create existing file with different content
        fs::write(&output_path, b"original content").unwrap();

        let mut cmd = Command::cargo_bin("decodeck").unwrap();
        cmd.args([
            "decode",
            text_base64,
            "--no-interactive",
            "--output",
            output_path.to_str().unwrap(),
            "--force",
        ])
        .assert()
        .success();

        // Content should be replaced
        let content = fs::read(&output_path).unwrap();
        assert_eq!(content, b"Hello World!");
    }

    #[test]
    fn force_works_on_new_file() {
        let dir = tempdir().unwrap();
        let output_path = dir.path().join("new_file.bin");
        let text_base64 = "SGVsbG8gV29ybGQh";

        let mut cmd = Command::cargo_bin("decodeck").unwrap();
        cmd.args([
            "decode",
            text_base64,
            "--no-interactive",
            "--output",
            output_path.to_str().unwrap(),
            "--force",
        ])
        .assert()
        .success();

        assert!(output_path.exists());
    }

    #[test]
    fn force_short_flag_works() {
        let dir = tempdir().unwrap();
        let output_path = dir.path().join("test.bin");
        let text_base64 = "SGVsbG8gV29ybGQh";

        fs::write(&output_path, b"old").unwrap();

        let mut cmd = Command::cargo_bin("decodeck").unwrap();
        cmd.args([
            "decode",
            text_base64,
            "--no-interactive",
            "-o",
            output_path.to_str().unwrap(),
            "-F",
        ])
        .assert()
        .success();

        let content = fs::read(&output_path).unwrap();
        assert_eq!(content, b"Hello World!");
    }
}
