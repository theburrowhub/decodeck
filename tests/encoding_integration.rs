//! Integration tests for multi-encoding support

use assert_cmd::Command;
use predicates::prelude::*;

fn decodeck() -> Command {
    Command::cargo_bin("decodeck").unwrap()
}

mod hex_encoding {
    use super::*;

    #[test]
    fn test_hex_with_0x_prefix() {
        let mut cmd = decodeck();
        cmd.args(["decode", "0x48656c6c6f", "--no-interactive"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Encoding: hex"))
            .stdout(predicate::str::contains("high confidence"));
    }

    #[test]
    fn test_hex_uppercase_prefix() {
        let mut cmd = decodeck();
        cmd.args(["decode", "0X48656C6C6F", "--no-interactive"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Encoding: hex"));
    }

    #[test]
    fn test_hex_explicit_flag() {
        let mut cmd = decodeck();
        cmd.args(["decode", "48656c6c6f", "-e", "hex", "--no-interactive"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Encoding: hex"))
            .stdout(predicate::str::contains("(specified)"));
    }

    #[test]
    fn test_hex_with_spaces() {
        let mut cmd = decodeck();
        cmd.args(["decode", "48 65 6c 6c 6f", "-e", "hex", "--no-interactive"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Size: 5 bytes"));
    }

    #[test]
    fn test_hex_json_output() {
        let mut cmd = decodeck();
        cmd.args(["decode", "0x48656c6c6f", "--json", "--no-interactive"])
            .assert()
            .success()
            .stdout(predicate::str::contains("\"type\": \"hex\""))
            .stdout(predicate::str::contains("\"confidence\": \"high\""));
    }
}

mod base32_encoding {
    use super::*;

    #[test]
    fn test_base32_auto_detect() {
        let mut cmd = decodeck();
        // "Hello World" in Base32
        cmd.args(["decode", "JBSWY3DPEBLW64TMMQ", "--no-interactive"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Encoding: base32"))
            .stdout(predicate::str::contains("medium confidence"));
    }

    #[test]
    fn test_base32_explicit_flag() {
        let mut cmd = decodeck();
        cmd.args(["decode", "JBSWY3DP", "-e", "base32", "--no-interactive"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Encoding: base32"))
            .stdout(predicate::str::contains("(specified)"));
    }

    #[test]
    fn test_base32_lowercase() {
        let mut cmd = decodeck();
        cmd.args(["decode", "jbswy3dpeblw64tmmq", "-e", "base32", "--no-interactive"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Size: 11 bytes"));
    }

    #[test]
    fn test_base32_with_padding() {
        let mut cmd = decodeck();
        cmd.args(["decode", "JBSWY3DPEBLW64TMMQ======", "-e", "base32", "--no-interactive"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Size: 11 bytes"));
    }
}

mod url_encoding {
    use super::*;

    #[test]
    fn test_url_percent_encoding() {
        let mut cmd = decodeck();
        cmd.args(["decode", "Hello%20World%21", "--no-interactive"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Encoding: url"))
            .stdout(predicate::str::contains("high confidence"));
    }

    #[test]
    fn test_url_explicit_flag() {
        let mut cmd = decodeck();
        cmd.args(["decode", "test%2Fpath", "-e", "url", "--no-interactive"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Encoding: url"))
            .stdout(predicate::str::contains("(specified)"));
    }

    #[test]
    fn test_url_plus_as_space() {
        let mut cmd = decodeck();
        cmd.args(["decode", "Hello+World", "-e", "url", "--no-interactive"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Size: 11 bytes"));
    }

    #[test]
    fn test_url_json_output() {
        let mut cmd = decodeck();
        cmd.args(["decode", "Hello%20World", "--json", "--no-interactive"])
            .assert()
            .success()
            .stdout(predicate::str::contains("\"type\": \"url\""));
    }
}

mod base85_encoding {
    use super::*;

    #[test]
    fn test_base85_with_delimiters() {
        let mut cmd = decodeck();
        cmd.args(["decode", "<~87cURD]j~>", "--no-interactive"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Encoding: base85"))
            .stdout(predicate::str::contains("high confidence"));
    }

    #[test]
    fn test_base85_explicit_flag() {
        let mut cmd = decodeck();
        cmd.args(["decode", "FCfN8", "-e", "base85", "--no-interactive"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Encoding: base85"))
            .stdout(predicate::str::contains("(specified)"));
    }

    #[test]
    fn test_base85_json_output() {
        let mut cmd = decodeck();
        cmd.args(["decode", "<~87cURD]j~>", "--json", "--no-interactive"])
            .assert()
            .success()
            .stdout(predicate::str::contains("\"type\": \"base85\""))
            .stdout(predicate::str::contains("\"confidence\": \"high\""));
    }
}

mod base64_encoding {
    use super::*;

    #[test]
    fn test_base64_default_fallback() {
        let mut cmd = decodeck();
        cmd.args(["decode", "SGVsbG8gV29ybGQ=", "--no-interactive"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Base64"))
            .stdout(predicate::str::contains("low confidence"));
    }

    #[test]
    fn test_base64_explicit_flag() {
        let mut cmd = decodeck();
        cmd.args(["decode", "SGVsbG8=", "-e", "base64", "--no-interactive"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Base64"))
            .stdout(predicate::str::contains("(specified)"));
    }

    #[test]
    fn test_base64_shows_variant() {
        let mut cmd = decodeck();
        cmd.args(["decode", "SGVsbG8=", "--no-interactive"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Standard Base64"));
    }
}

mod auto_detection {
    use super::*;

    #[test]
    fn test_detection_priority_hex_prefix() {
        // 0x prefix should be detected as hex with high confidence
        let mut cmd = decodeck();
        cmd.args(["decode", "0x4142", "--json", "--no-interactive"])
            .assert()
            .success()
            .stdout(predicate::str::contains("\"type\": \"hex\""))
            .stdout(predicate::str::contains("\"confidence\": \"high\""));
    }

    #[test]
    fn test_detection_priority_base85_delimiters() {
        // <~ ~> delimiters should be detected as base85 with high confidence
        let mut cmd = decodeck();
        cmd.args(["decode", "<~FCfN8~>", "--json", "--no-interactive"])
            .assert()
            .success()
            .stdout(predicate::str::contains("\"type\": \"base85\""))
            .stdout(predicate::str::contains("\"confidence\": \"high\""));
    }

    #[test]
    fn test_detection_priority_url_percent() {
        // %XX sequences should be detected as URL with high confidence
        let mut cmd = decodeck();
        cmd.args(["decode", "test%20value", "--json", "--no-interactive"])
            .assert()
            .success()
            .stdout(predicate::str::contains("\"type\": \"url\""))
            .stdout(predicate::str::contains("\"confidence\": \"high\""));
    }
}
