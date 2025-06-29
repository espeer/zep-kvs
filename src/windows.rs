//! Windows-specific storage implementation using the Windows Registry.
//!
//! This module implements storage scopes for Windows systems using the
//! Windows Registry as the backing store. Data is stored as binary values
//! in registry keys under appropriate hives for user and machine scope.

use winreg::RegKey;
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_SET_VALUE, RegType};
use winreg::reg_key::HKEY;
use winreg::reg_value::RegValue;

use crate::api::scope::{Machine, User};
use crate::api::{BackingStore, Scope};
use crate::error::KvsError;

use std::io::ErrorKind;
use std::path::PathBuf;

/// Windows Registry-based key-value store.
///
/// This store uses the Windows Registry to persist key-value pairs.
/// Data is stored as binary registry values under a structured key path
/// that includes the package name and application name.
///
/// # Registry Structure
///
/// ```text
/// HKEY_CURRENT_USER (or HKEY_LOCAL_MACHINE)
/// └── Software
///     └── {package_name}
///         └── {app_name}
///             ├── key1 = binary_data
///             ├── key2 = binary_data
///             └── ...
/// ```
///
/// # Data Storage
///
/// All values are stored as `REG_BINARY` type to handle arbitrary byte data.
/// This allows the store to handle any serializable data type consistently.
pub struct RegistryStore {
    /// The registry hive (HKEY_CURRENT_USER or HKEY_LOCAL_MACHINE)
    scope: HKEY,
    /// The registry path relative to the hive root
    path: PathBuf,
}

impl RegistryStore {
    /// Creates a new registry store for the specified hive.
    ///
    /// This method constructs the registry path using the package name and
    /// application name, then creates the necessary registry keys if they
    /// don't already exist.
    ///
    /// # Arguments
    ///
    /// * `scope` - The registry hive to use (HKEY_CURRENT_USER or HKEY_LOCAL_MACHINE)
    ///
    /// # Registry Path
    ///
    /// The created path follows the pattern:
    /// `{scope}\Software\{package_name}\{app_name}`
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The process lacks permissions to create registry keys
    /// - Registry access fails for other reasons
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zep_kvs::windows::RegistryStore;
    /// # use winreg::enums::HKEY_CURRENT_USER;
    /// let store = RegistryStore::new(HKEY_CURRENT_USER)?;
    /// # Ok::<(), zep_kvs::error::KvsError>(())
    /// ```
    pub(crate) fn new(scope: HKEY) -> Result<Self, KvsError> {
        let path = PathBuf::new()
            .join("Software")
            .join(env!("CARGO_PKG_NAME"))
            .join(env!("ZEP_KVS_APP_NAME"));
        let result = Self { scope, path };
        RegKey::predef(result.scope)
            .create_subkey(&result.path)
            .map_err(|e| KvsError::io_at(e, &result.full_path()))?;
        Ok(result)
    }

    /// Returns the full registry path for error reporting.
    ///
    /// Constructs a human-readable path string that includes the hive name
    /// and the relative path for use in error messages.
    fn full_path(&self) -> PathBuf {
        match self.scope {
            HKEY_CURRENT_USER => PathBuf::from("winreg:HKEY_CURRENT_USER"),
            HKEY_LOCAL_MACHINE => PathBuf::from("winreg:HKEY_LOCAL_MACHINE"),
            _ => unreachable!(),
        }
        .join(self.path.clone())
    }

    /// Sets a registry value as binary data.
    ///
    /// Stores the provided bytes as a REG_BINARY value under the given key name.
    /// Opens the registry key with write permissions and sets the value atomically.
    ///
    /// # Arguments
    ///
    /// * `key` - The value name to store data under
    /// * `value` - The binary data to store
    ///
    /// # Errors
    ///
    /// Returns an I/O error if registry access fails or if the process
    /// lacks permissions to write to the registry key.
    fn set_value(&self, key: &str, value: &[u8]) -> Result<(), std::io::Error> {
        let value = RegValue {
            bytes: value.to_owned(),
            vtype: RegType::REG_BINARY,
        };
        RegKey::predef(self.scope)
            .open_subkey_with_flags(&self.path, KEY_SET_VALUE)?
            .set_raw_value(key, &value)
    }

    /// Retrieves a registry value as binary data.
    ///
    /// Attempts to read the registry value for the given key name.
    /// Returns `None` if the value doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `key` - The value name to retrieve
    ///
    /// # Returns
    ///
    /// - `Ok(Some(bytes))` - The binary data if the value exists
    /// - `Ok(None)` - If the value doesn't exist
    /// - `Err(error)` - If registry access fails
    fn get_value(&self, key: &str) -> Result<Option<Vec<u8>>, std::io::Error> {
        match RegKey::predef(self.scope)
            .open_subkey(&self.path)?
            .get_raw_value(key)
        {
            Ok(value) => Ok(Some(value.bytes)),
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Deletes a registry value.
    ///
    /// Removes the specified value name from the registry key.
    /// Does nothing if the value doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `key` - The value name to delete
    ///
    /// # Errors
    ///
    /// Returns an I/O error if registry access fails or if the process
    /// lacks permissions to modify the registry key.
    fn delete_value(&self, key: &str) -> Result<(), std::io::Error> {
        RegKey::predef(self.scope)
            .open_subkey_with_flags(&self.path, KEY_SET_VALUE)?
            .delete_value(key)?;
        Ok(())
    }
}

impl BackingStore for RegistryStore {
    fn keys(&self) -> Result<Vec<String>, KvsError> {
        Ok(RegKey::predef(self.scope)
            .open_subkey(&self.path)
            .map_err(|e| KvsError::io_at(e, &self.full_path()))?
            .enum_values()
            .filter_map(|r| r.ok())
            .map(|x| x.0)
            .collect())
    }

    fn store(&mut self, key: &str, value: &[u8]) -> Result<(), KvsError> {
        self.set_value(key, value)
            .map_err(|e| KvsError::io_at(e, &self.full_path()))
    }

    fn retrieve(&self, key: &str) -> Result<Option<Vec<u8>>, KvsError> {
        self.get_value(key)
            .map_err(|e| KvsError::io_at(e, &self.full_path()))
    }

    fn remove(&mut self, key: &str) -> Result<(), KvsError> {
        self.delete_value(key)
            .map_err(|e| KvsError::io_at(e, &self.full_path()))
    }
}

impl Scope for Machine {
    type Store = RegistryStore;

    /// Creates a machine-wide storage scope for Windows.
    ///
    /// Uses `HKEY_LOCAL_MACHINE` registry hive for system-wide application data.
    /// This scope stores data that is available to all users on the system.
    ///
    /// # Storage Location
    ///
    /// Data is stored in:
    /// `HKEY_LOCAL_MACHINE\Software\{package_name}\{app_name}\`
    ///
    /// # Permissions
    ///
    /// Writing to `HKEY_LOCAL_MACHINE` typically requires:
    /// - Administrator privileges, or
    /// - Running as a service with appropriate permissions, or
    /// - Proper application manifest with elevated execution level
    ///
    /// # Errors
    ///
    /// Returns `NoMachineScope` if:
    /// - The process lacks permissions to create or write to registry keys in HKLM
    /// - Registry access is restricted by security policies
    /// - The registry operation fails for other reasons
    fn new() -> Result<Self::Store, KvsError> {
        RegistryStore::new(HKEY_LOCAL_MACHINE)
    }
}

impl Scope for User {
    type Store = RegistryStore;

    /// Creates a user-specific storage scope for Windows.
    ///
    /// Uses `HKEY_CURRENT_USER` registry hive for user-specific application data.
    /// This scope stores data that is only available to the current user.
    ///
    /// # Storage Location
    ///
    /// Data is stored in:
    /// `HKEY_CURRENT_USER\Software\{package_name}\{app_name}\`
    ///
    /// # Permissions
    ///
    /// The current user typically has full access to their HKEY_CURRENT_USER hive,
    /// so no special permissions are required for most operations.
    ///
    /// # User Profile Considerations
    ///
    /// - Data follows the user across different machines in domain environments
    /// - Data is included in user profile backups and roaming profiles
    /// - Data is automatically cleaned up when the user profile is deleted
    ///
    /// # Errors
    ///
    /// Returns `NoUserScope` if:
    /// - Registry access fails due to security restrictions
    /// - The user profile is corrupted or inaccessible
    /// - The registry operation fails for other reasons
    fn new() -> Result<Self::Store, KvsError> {
        RegistryStore::new(HKEY_CURRENT_USER)
    }
}
