//! Interactive terminal module

use crate::metadata::ContentMetadata;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal,
};
use std::io::{IsTerminal, Write};

/// Interactive prompt handler
pub struct InteractivePrompt {
    /// Whether to show the prompt
    pub enabled: bool,
}

impl InteractivePrompt {
    /// Create new interactive prompt
    pub fn new(quiet: bool, no_interactive: bool) -> Self {
        let is_tty = std::io::stdout().is_terminal();
        Self {
            enabled: is_tty && !quiet && !no_interactive,
        }
    }

    /// Get prompt text based on content type
    pub fn prompt_text(metadata: &ContentMetadata) -> Option<&'static str> {
        if metadata.is_viewable {
            Some("Press space to view...")
        } else if metadata.is_playable {
            Some("Press space to play...")
        } else {
            None
        }
    }

    /// Show prompt and wait for user input
    /// Returns true if user pressed space or enter
    pub fn show_and_wait(&self, metadata: &ContentMetadata) -> bool {
        if !self.enabled {
            return false;
        }

        if let Some(prompt) = Self::prompt_text(metadata) {
            println!();
            print!("{}", prompt);
            let _ = std::io::stdout().flush();

            self.wait_for_key()
        } else {
            false
        }
    }

    /// Wait for key press and return true if space or enter
    fn wait_for_key(&self) -> bool {
        // Enable raw mode for key capture
        if terminal::enable_raw_mode().is_err() {
            return false;
        }

        let result = loop {
            if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
                break match code {
                    KeyCode::Char(' ') | KeyCode::Enter => true,
                    KeyCode::Char('q') | KeyCode::Esc => false,
                    _ => continue,
                };
            }
        };

        // Restore terminal mode
        let _ = terminal::disable_raw_mode();
        println!(); // New line after key press

        result
    }

    /// Open file with system default application
    pub fn open_file(path: &std::path::Path) -> Result<(), std::io::Error> {
        opener::open(path).map_err(std::io::Error::other)
    }
}
