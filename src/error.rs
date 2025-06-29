//! Error types for the Zep Key-Value Store library.
//!
//! This module defines all possible errors that can occur when using
//! the key-value store, including I/O errors, scope access issues,
//! and data conversion problems.

use std::path::{Path, PathBuf};

use thiserror::Error;

/// Errors that can occur when using the key-value store.
///
/// This enum covers all possible failure modes, from file system
/// access issues to data serialization problems.
#[derive(Error, Debug)]
pub enum KvsError {
    /// Error when stored data cannot be decoded as valid UTF-8.
    ///
    /// This typically occurs when trying to retrieve binary data
    /// as a string, or when stored data has been corrupted.
    #[error("Invalid UTF-8 InBytes")]
    StringDecodeError(#[from] std::string::FromUtf8Error),

    /// Error during data serialization or deserialization.
    ///
    /// This occurs when data cannot be properly converted to or from
    /// bytes for storage, such as invalid byte lengths or malformed data.
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// I/O error that occurred while accessing storage.
    ///
    /// This includes file system errors, permission issues,
    /// and other low-level storage problems. The `path` field
    /// indicates where the error occurred.
    #[error("{source}: {path}")]
    IoError {
        /// The file system path where the error occurred.
        path: PathBuf,
        /// The underlying I/O error.
        source: std::io::Error,
    },

    /// Machine-wide storage scope is not available.
    ///
    /// This typically occurs when the application lacks the necessary
    /// permissions to access system-wide storage locations, or when
    /// the required directories cannot be created.
    #[error("No machine scope. {0}")]
    NoMachineScope(String),

    /// User-specific storage scope is not available.
    ///
    /// This can happen when the user's home directory is not accessible,
    /// when environment variables are missing, or when user directories
    /// cannot be created due to permission issues.
    #[error("No user scope. {0}")]
    NoUserScope(String),
}

impl KvsError {
    /// Creates an I/O error with location context.
    ///
    /// This is a convenience method used internally to wrap
    /// standard I/O errors with path information for better
    /// error reporting.
    ///
    /// # Arguments
    ///
    /// * `io` - The underlying I/O error
    /// * `at` - The path where the error occurred
    pub(crate) fn io_at(io: std::io::Error, at: &Path) -> KvsError {
        KvsError::IoError {
            source: io,
            path: at.to_path_buf(),
        }
    }
}
