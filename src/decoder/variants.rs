//! Base64 variant detection and handling

use super::Base64Variant;

/// Detect Base64 variant from input string
pub fn detect_variant(input: &str) -> Base64Variant {
    // URL-safe uses - and _ instead of + and /
    if input.contains('-') || input.contains('_') {
        Base64Variant::UrlSafe
    } else {
        Base64Variant::Standard
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_standard() {
        assert_eq!(detect_variant("SGVsbG8rV29ybGQ="), Base64Variant::Standard);
    }

    #[test]
    fn test_detect_urlsafe() {
        assert_eq!(detect_variant("SGVsbG8tV29ybGQ_"), Base64Variant::UrlSafe);
    }
}
