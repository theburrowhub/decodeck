//! Decodeck - Multi-encoding decoding library with metadata detection
//!
//! This library provides functionality for:
//! - Decoding multiple formats: Base64, Hex, Base32, URL, Ascii85
//! - Auto-detecting encoding types
//! - Detecting content MIME types via magic bytes
//! - Formatting output in text and JSON formats

pub mod decoder;
pub mod encoding;
pub mod error;
pub mod input;
pub mod interactive;
pub mod metadata;
pub mod output;

pub use error::DecodeckError;

use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Initialize the tracing subscriber for logging
pub fn init_logging(verbose: bool) {
    let filter = if verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::new("info")
    };

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();
}
