//! Input source implementations

use super::{InputSource, SourceType};
use crate::error::DecodeckError;
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

/// Default stdin timeout in seconds
pub const STDIN_TIMEOUT_SECS: u64 = 5;

impl InputSource {
    /// Create InputSource from CLI argument
    pub fn from_arg(data: &str) -> Result<Self, DecodeckError> {
        Ok(Self::new(SourceType::Arg, data.as_bytes().to_vec(), None))
    }

    /// Create InputSource from file path
    pub fn from_file(path: &Path) -> Result<Self, DecodeckError> {
        let data = std::fs::read(path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                DecodeckError::FileNotFound {
                    path: path.display().to_string(),
                }
            } else if e.kind() == std::io::ErrorKind::PermissionDenied {
                DecodeckError::PermissionDenied {
                    path: path.display().to_string(),
                }
            } else {
                DecodeckError::Io(e)
            }
        })?;
        Ok(Self::new(SourceType::File, data, Some(path.to_path_buf())))
    }

    /// Create InputSource from stdin with timeout
    pub fn from_stdin() -> Result<Self, DecodeckError> {
        Self::from_stdin_with_timeout(Duration::from_secs(STDIN_TIMEOUT_SECS))
    }

    /// Create InputSource from stdin with custom timeout
    pub fn from_stdin_with_timeout(timeout: Duration) -> Result<Self, DecodeckError> {
        use std::io::Read;

        let (tx, rx) = mpsc::channel();

        // Spawn thread to read stdin
        std::thread::spawn(move || {
            let mut data = Vec::new();
            let result = std::io::stdin().read_to_end(&mut data);
            let _ = tx.send((data, result));
        });

        // Wait with timeout
        match rx.recv_timeout(timeout) {
            Ok((data, Ok(_))) => {
                if data.is_empty() {
                    return Err(DecodeckError::NoInput);
                }
                Ok(Self::new(SourceType::Stdin, data, None))
            }
            Ok((_, Err(e))) => Err(DecodeckError::Io(e)),
            Err(mpsc::RecvTimeoutError::Timeout) => Err(DecodeckError::SystemError {
                message: format!(
                    "Stdin read timed out after {} seconds. Pipe data or use -f flag.",
                    timeout.as_secs()
                ),
            }),
            Err(mpsc::RecvTimeoutError::Disconnected) => Err(DecodeckError::SystemError {
                message: "Stdin reader disconnected unexpectedly".to_string(),
            }),
        }
    }
}
