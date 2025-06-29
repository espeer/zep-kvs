//! File system-based storage implementation for persistent data.
//!
//! This module provides a directory-based storage backend that persists
//! data to the file system. Each key-value pair is stored as a separate
//! file within a dedicated directory structure.

use std::fs;
use std::fs::File;
use std::io::{ErrorKind, Write};
use std::path::PathBuf;
use std::time::Duration;

use rand::random;

use crate::api::BackingStore;
use crate::error::KvsError;

const TEMP_PREFIX: &str = ".tmp_";

/// File system-based key-value store.
///
/// This store persists data by creating individual files for each key
/// within a dedicated directory. It uses atomic writes via a temporary
/// directory to ensure data consistency.
///
/// # Storage Structure
///
/// ```text
/// base_directory/
/// ├── key1              # File containing value for "key1"
/// ├── key2              # File containing value for "key2"
/// └── .tmp_random_id    # Temporary files during atomic writes
/// ```
///
/// # Atomic Writes
///
/// The store uses temporary files with random names to ensure atomic writes.
/// Data is first written to a temporary file, then atomically renamed to the
/// final key file to prevent corruption during concurrent access.
pub struct DirectoryStore {
    /// The base directory where key files are stored.
    path: PathBuf,
    /// File handle for the base directory, used for sync.
    dir: File,
}

impl DirectoryStore {
    /// Creates a new directory store at the specified path.
    ///
    /// This method:
    /// 1. Creates the directory structure if it doesn't exist
    /// 2. Cleans up stale temporary files older than 24 hours
    /// 3. Opens the directory for sync operations
    ///
    /// # Arguments
    ///
    /// * `path` - Base path where the store directory should be created.
    ///           The actual storage directory will be `path/package_name/app_name`.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Directory creation fails due to permissions
    /// - Directory cannot be opened
    /// - Cleanup of stale temporary files fails
    pub(crate) fn new(path: PathBuf) -> Result<Self, KvsError> {
        let path = path
            .join(env!("CARGO_PKG_NAME"))
            .join(env!("ZEP_KVS_APP_NAME"));

        let remove_stale = || {
            fs::create_dir_all(&path)?; // Ensure directory exists
            fs::read_dir(&path)?
                .filter_map(|d| d.ok()) // Skip entries with errors
                .filter(|d| {
                    d.file_type().is_ok_and(|f| f.is_file())
                        && d.file_name()
                            .to_str()
                            .is_some_and(|s| s.starts_with(TEMP_PREFIX))
                }) // Only include temporary files
                .filter(|d| {
                    d.metadata().is_ok_and(|m| {
                        m.modified().is_ok_and(|t| {
                            t.elapsed().is_ok_and(|d| d > Duration::from_secs(86400))
                        })
                    })
                }) // Only include files older than 24 hours
                .for_each(|d| {
                    let _ = fs::remove_file(d.path());
                });
            let dir = File::open(&path)?;
            dir.sync_all()?;
            Ok(dir)
        };
        let dir = remove_stale().map_err(|e| KvsError::io_at(e, &path))?;
        Ok(Self { path, dir })
    }
}

impl BackingStore for DirectoryStore {
    fn keys(&self) -> Result<Vec<String>, KvsError> {
        // Read directory entries and filter for regular files
        Ok(fs::read_dir(&self.path)
            .map_err(|e| KvsError::io_at(e, &self.path))?
            .filter_map(|d| d.ok()) // Skip entries with errors
            .filter(|d| d.file_type().is_ok_and(|d| d.is_file())) // Only include files
            .filter_map(|f| f.file_name().to_str().map(|f| f.to_owned())) // Convert to strings
            .filter(|k| !k.starts_with(TEMP_PREFIX)) // Exclude temporary files
            .collect())
    }

    fn store(&mut self, key: &str, value: &[u8]) -> Result<(), KvsError> {
        let path = self.path.join(key);
        let result = || {
            // Create temporary file with unique name
            let tmp = self.path.join(format!("{TEMP_PREFIX}{}", random::<u128>()));
            let mut file = File::create_new(&tmp)?;

            // Write data and ensure it's flushed to disk
            file.write_all(value)?;
            file.sync_all()?;

            // Atomically move temporary file to final location
            fs::rename(tmp, &path)?;

            // Sync directory to ensure rename is persistent
            self.dir.sync_all()
        };
        result().map_err(|e| KvsError::io_at(e, &path))
    }

    fn retrieve(&self, key: &str) -> Result<Option<Vec<u8>>, crate::error::KvsError> {
        // Attempt to read the file for this key
        match fs::read(self.path.join(key)) {
            Ok(value) => Ok(Some(value)),
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(None), // Key doesn't exist
            Err(e) => Err(KvsError::io_at(e, &self.path)),
        }
    }

    fn remove(&mut self, key: &str) -> Result<(), crate::error::KvsError> {
        let path = self.path.join(key);
        let result = || {
            // Remove the file for this key
            fs::remove_file(&path)?;
            // Sync directory to ensure removal is persistent
            self.dir.sync_all()
        };
        result().map_err(|e| KvsError::io_at(e, &path))
    }
}
