//! Error types for the Zep Key-Value Store library.
//!
//! This module defines all possible errors that can occur when using
//! the key-value store, including I/O errors, scope access issues,
//! and data conversion problems.

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
}
