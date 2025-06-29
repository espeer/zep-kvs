//! Linux-specific storage implementation using XDG Base Directory Specification.
//!
//! This module implements storage scopes for Linux systems, following
//! the XDG Base Directory Specification for user data and using `/var/lib`
//! for system-wide machine data.

use std::env;
use std::path::PathBuf;

use crate::api::Scope;
use crate::api::scope::{Machine, User};
use crate::directory::DirectoryStore;
use crate::error::KvsError;

impl Scope for Machine {
    type Store = DirectoryStore;

    /// Creates a machine-wide storage scope for Linux.
    ///
    /// Uses `/var/lib` as the base directory for system-wide application data.
    /// This location requires root privileges to write to and follows Linux
    /// conventions for system service data.
    ///
    /// # Storage Location
    ///
    /// Data is stored in `/var/lib/{package_name}/{app_name}/`
    ///
    /// # Errors
    ///
    /// Returns `NoMachineScope` if:
    /// - The process lacks permissions to create directories in `/var/lib`
    /// - The file system is read-only
    /// - Directory creation fails for other I/O reasons
    fn new() -> Result<Self::Store, KvsError> {
        DirectoryStore::new(PathBuf::from("/var/lib"))
            .map_err(|e| KvsError::NoMachineScope(e.to_string()))
    }
}

impl Scope for User {
    type Store = DirectoryStore;

    /// Creates a user-specific storage scope for Linux.
    ///
    /// Follows the XDG Base Directory Specification:
    /// 1. First tries `$XDG_DATA_HOME` if set
    /// 2. Falls back to `$HOME/.local/share` if `$HOME` is available
    ///
    /// # Storage Location
    ///
    /// Data is stored in one of:
    /// - `$XDG_DATA_HOME/{package_name}/{app_name}/` (if `XDG_DATA_HOME` is set)
    /// - `$HOME/.local/share/{package_name}/{app_name}/` (fallback)
    ///
    /// # Environment Variables
    ///
    /// - `XDG_DATA_HOME` - Primary location for user data files
    /// - `HOME` - User's home directory (fallback)
    ///
    /// # Errors
    ///
    /// Returns `NoUserScope` if:
    /// - Neither `XDG_DATA_HOME` nor `HOME` environment variables are set
    /// - The user lacks permissions to create directories in the target location
    /// - Directory creation fails for other I/O reasons
    fn new() -> Result<Self::Store, KvsError> {
        let path = env::var_os("XDG_DATA_HOME")
            .map(|d| PathBuf::from(d))
            .or(env::var_os("HOME").map(|d| PathBuf::from(d).join(".local/share")));
        match path {
            Some(path) => {
                DirectoryStore::new(path).map_err(|e| KvsError::NoUserScope(e.to_string()))
            }
            None => Err(KvsError::NoUserScope("no user directory found".to_string())),
        }
    }
}
