//! macOS-specific storage implementation using standard Application Support directories.
//!
//! This module implements storage scopes for macOS systems, following
//! Apple's recommended conventions for application data storage in
//! the Library/Application Support hierarchy.

use std::env;
use std::path::PathBuf;

use crate::api::Scope;
use crate::api::scope::{Machine, User};
use crate::directory::DirectoryStore;
use crate::error::KvsError;

impl Scope for Machine {
    type Store = DirectoryStore;

    /// Creates a machine-wide storage scope for macOS.
    ///
    /// Uses `/Library/Application Support` as the base directory for system-wide
    /// application data. This location follows Apple's guidelines for shared
    /// application data and typically requires administrator privileges to write to.
    ///
    /// # Storage Location
    ///
    /// Data is stored in `/Library/Application Support/{package_name}/{app_name}/`
    ///
    /// # Permissions
    ///
    /// Writing to `/Library/Application Support` typically requires:
    /// - Administrator privileges, or
    /// - Proper application signing and entitlements
    ///
    /// # Errors
    ///
    /// Returns `NoMachineScope` if:
    /// - The process lacks permissions to create directories in `/Library/Application Support`
    /// - The file system is read-only
    /// - Directory creation fails for other I/O reasons
    fn new() -> Result<Self::Store, KvsError> {
        // Use /Library/Application Support for system-wide storage on macOS
        DirectoryStore::new(PathBuf::from("/Library/Application Support"))
            .map_err(|e| KvsError::NoMachineScope(e.to_string()))
    }
}

impl Scope for User {
    type Store = DirectoryStore;

    /// Creates a user-specific storage scope for macOS.
    ///
    /// Uses `~/Library/Application Support` as the base directory for user-specific
    /// application data. This follows Apple's Human Interface Guidelines and
    /// File System Programming Guide recommendations for application data storage.
    ///
    /// # Storage Location
    ///
    /// Data is stored in `$HOME/Library/Application Support/{package_name}/{app_name}/`
    ///
    /// # Environment Variables
    ///
    /// - `HOME` - User's home directory (required)
    ///
    /// # macOS Conventions
    ///
    /// The `~/Library/Application Support` directory is:
    /// - Automatically backed up by Time Machine
    /// - Hidden from users in Finder by default
    /// - The recommended location for application data files
    /// - Automatically created by macOS if it doesn't exist
    ///
    /// # Errors
    ///
    /// Returns `NoUserScope` if:
    /// - The `HOME` environment variable is not set
    /// - The user lacks permissions to create directories in `~/Library/Application Support`
    /// - Directory creation fails for other I/O reasons
    fn new() -> Result<Self::Store, KvsError> {
        // Use ~/Library/Application Support for user-specific storage on macOS
        let path = env::var_os("HOME").map(|home| {
            PathBuf::from(home)
                .join("Library")
                .join("Application Support")
        });

        match path {
            Some(path) => {
                DirectoryStore::new(path).map_err(|e| KvsError::NoUserScope(e.to_string()))
            }
            None => Err(KvsError::NoUserScope("no user directory found".to_string())),
        }
    }
}
