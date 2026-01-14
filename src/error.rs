//! Error types for decodeck

use thiserror::Error;

/// All possible errors in decodeck
#[derive(Error, Debug)]
pub enum DecodeckError {
    #[error("No input data provided")]
    NoInput,

    #[error("Invalid Base64: {message} at position {position}")]
    InvalidBase64 { message: String, position: usize },

    #[error("File not found: {path}")]
    FileNotFound { path: String },

    #[error("Permission denied: {path}")]
    PermissionDenied { path: String },

    #[error("Input size {actual} exceeds limit {limit}")]
    SizeExceeded { actual: String, limit: String },

    #[error("Output file already exists: {path}")]
    OutputExists { path: String },

    #[error("Invalid output path: {path} - {reason}")]
    InvalidOutputPath { path: String, reason: String },

    #[error("Decode failed: {message}")]
    DecodeFailed { message: String },

    #[error("System error: {message}")]
    SystemError { message: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Exit codes per specification
pub mod exit_codes {
    pub const SUCCESS: i32 = 0;
    pub const USER_ERROR: i32 = 1;
    pub const SYSTEM_ERROR: i32 = 2;
}
