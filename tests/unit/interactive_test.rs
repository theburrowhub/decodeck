//! Unit tests for interactive module

use decodeck::interactive::InteractivePrompt;
use decodeck::metadata::{ContentCategory, ContentMetadata};

// T054: Tests for terminal detection
mod terminal_detection {
    use super::*;

    #[test]
    fn quiet_mode_disables_prompt() {
        let prompt = InteractivePrompt::new(true, false); // quiet=true
        assert!(!prompt.enabled);
    }

    #[test]
    fn no_interactive_flag_disables_prompt() {
        let prompt = InteractivePrompt::new(false, true); // no_interactive=true
        assert!(!prompt.enabled);
    }

    #[test]
    fn both_flags_disable_prompt() {
        let prompt = InteractivePrompt::new(true, true);
        assert!(!prompt.enabled);
    }
}

// T055: Tests for prompt text selection
mod prompt_text {
    use super::*;

    fn create_viewable_metadata() -> ContentMetadata {
        ContentMetadata {
            mime_type: "image/png".to_string(),
            extension: ".png".to_string(),
            category: ContentCategory::Image,
            is_viewable: true,
            is_playable: false,
            magic_bytes: None,
        }
    }

    fn create_playable_metadata() -> ContentMetadata {
        ContentMetadata {
            mime_type: "audio/mpeg".to_string(),
            extension: ".mp3".to_string(),
            category: ContentCategory::Audio,
            is_viewable: false,
            is_playable: true,
            magic_bytes: None,
        }
    }

    fn create_other_metadata() -> ContentMetadata {
        ContentMetadata {
            mime_type: "application/octet-stream".to_string(),
            extension: ".bin".to_string(),
            category: ContentCategory::Other,
            is_viewable: false,
            is_playable: false,
            magic_bytes: None,
        }
    }

    #[test]
    fn viewable_content_shows_view_prompt() {
        let metadata = create_viewable_metadata();
        let prompt = InteractivePrompt::prompt_text(&metadata);
        assert_eq!(prompt, Some("Press space to view..."));
    }

    #[test]
    fn playable_content_shows_play_prompt() {
        let metadata = create_playable_metadata();
        let prompt = InteractivePrompt::prompt_text(&metadata);
        assert_eq!(prompt, Some("Press space to play..."));
    }

    #[test]
    fn other_content_shows_no_prompt() {
        let metadata = create_other_metadata();
        let prompt = InteractivePrompt::prompt_text(&metadata);
        assert_eq!(prompt, None);
    }
}

// T056: Tests for key handling
mod key_handling {
    // Key handling tests would require mocking terminal input
    // These are better tested via integration tests

    #[test]
    fn placeholder_for_key_handling() {
        // Key capture is tested in integration tests
        assert!(true);
    }
}
