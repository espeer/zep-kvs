//! Core API for the Zep Key-Value Store library.
//!
//! This module provides the main interfaces for storing and retrieving data
//! across different scopes (User, Machine, Ephemeral) on various platforms.

use std::convert::AsRef;

use crate::convert::{InBytes, OutBytes};
use crate::error::KvsError;

/// Defines a storage scope for key-value data.
///
/// Each scope determines where data is stored and how it persists.
/// The associated `Store` type implements the actual storage mechanism
/// appropriate for the platform and scope.
pub trait Scope {
    /// The backing store implementation for this scope.
    type Store: BackingStore;

    /// Creates a new store instance for this scope.
    ///
    /// # Errors
    ///
    /// Returns an error if the storage location cannot be accessed or created.
    fn new() -> Result<Self::Store, KvsError>;
}

/// Available storage scopes for key-value data.
pub mod scope {
    /// In-memory storage that doesn't persist between program runs.
    ///
    /// This scope is useful for testing and temporary data that should
    /// not be written to disk.
    pub struct Ephemeral();

    /// System-wide storage shared across all users.
    ///
    /// On Unix systems, this typically requires root privileges.
    /// On Windows, this requires administrator privileges.
    /// Data stored in this scope is available to all users of the system.
    pub struct Machine();

    /// User-specific storage that persists between program runs.
    ///
    /// Data is stored in the current user's profile directory
    /// following platform conventions:
    /// - Linux: `$XDG_DATA_HOME` or `~/.local/share`
    /// - macOS: `~/Library/Application Support`
    /// - Windows: `HKEY_CURRENT_USER\Software`
    pub struct User();
}

/// A type-safe key-value store with configurable storage scope.
///
/// This is the main interface for storing and retrieving data. The generic
/// parameter `S` determines the storage scope (User, Machine, or Ephemeral).
///
/// # Examples
///
/// ```
/// use zep_kvs::prelude::*;
///
/// // Create a user-scoped store
/// let mut store = KeyValueStore::<scope::User>::new()?;
///
/// // Store some data
/// store.store("name", "alice")?;
/// store.store("count", 42u32)?;
///
/// // Retrieve data
/// let username: String = store.retrieve("name")?.unwrap();
/// let count: u32 = store.retrieve("count")?.unwrap();
///
/// // Remove data
/// store.remove("name")?;
/// store.remove("count")?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct KeyValueStore<S: Scope> {
    inner: S::Store,
}

impl<S: Scope> KeyValueStore<S> {
    /// Creates a new key-value store for the specified scope.
    ///
    /// # Errors
    ///
    /// Returns an error if the storage backend cannot be initialized,
    /// typically due to permission issues or missing directories.
    ///
    /// # Examples
    ///
    /// ```
    /// use zep_kvs::prelude::*;
    ///
    /// let store = KeyValueStore::<scope::Ephemeral>::new()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new() -> Result<Self, KvsError> {
        Ok(Self { inner: S::new()? })
    }

    /// Returns all keys currently stored in this store.
    ///
    /// # Errors
    ///
    /// Returns an error if the storage backend cannot be accessed.
    ///
    /// # Examples
    ///
    /// ```
    /// use zep_kvs::prelude::*;
    ///
    /// let mut store = KeyValueStore::<scope::Ephemeral>::new()?;
    /// store.store("key1", "value1")?;
    /// store.store("key2", "value2")?;
    ///
    /// let keys = store.keys()?;
    /// assert_eq!(keys.len(), 2);
    /// assert!(keys.contains(&"key1".to_string()));
    /// assert!(keys.contains(&"key2".to_string()));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn keys(&self) -> Result<Vec<String>, KvsError> {
        self.inner.keys()
    }

    /// Stores a value under the given key.
    ///
    /// If the key already exists, its value will be overwritten.
    /// The value can be any type that implements `OutBytes`, including
    /// strings, integers, and byte arrays.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to store the value under. Can be any type that
    ///           converts to a string reference.
    /// * `value` - The value to store. Must implement `OutBytes`.
    ///
    /// # Errors
    ///
    /// Returns an error if the value cannot be serialized or if the
    /// storage backend fails to write the data.
    ///
    /// # Examples
    ///
    /// ```
    /// use zep_kvs::prelude::*;
    ///
    /// let mut store = KeyValueStore::<scope::Ephemeral>::new()?;
    ///
    /// // Store different types
    /// store.store("name", "Alice")?;
    /// store.store("age", 30u32)?;
    /// store.store("data", vec![1u8, 2u8, 3u8].as_slice())?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn store<K: AsRef<str>, V: OutBytes>(&mut self, key: K, value: V) -> Result<(), KvsError> {
        self.inner.store(key.as_ref(), &value.out_bytes()?)
    }

    /// Retrieves a value by key, if it exists.
    ///
    /// Returns `None` if the key is not found. The return type must be
    /// specified and implement `InBytes` for deserialization.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up. Can be any type that converts to a string reference.
    ///
    /// # Type Parameters
    ///
    /// * `V` - The expected type of the stored value. Must implement `InBytes`.
    ///
    /// # Errors
    ///
    /// Returns an error if the storage backend fails to read the data
    /// or if the stored data cannot be deserialized to the requested type.
    ///
    /// # Examples
    ///
    /// ```
    /// use zep_kvs::prelude::*;
    ///
    /// let mut store = KeyValueStore::<scope::Ephemeral>::new()?;
    /// store.store("count", 42u32)?;
    ///
    /// // Retrieve with explicit type annotation
    /// let count: u32 = store.retrieve("count")?.unwrap();
    /// assert_eq!(count, 42);
    ///
    /// // Check for non-existent key
    /// let missing: Option<String> = store.retrieve("missing")?;
    /// assert!(missing.is_none());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn retrieve<K: AsRef<str>, V: InBytes>(&self, key: K) -> Result<Option<V>, KvsError> {
        Ok(match self.inner.retrieve(key.as_ref())? {
            Some(data) => Some(V::in_bytes(&data)?),
            None => None,
        })
    }

    /// Removes a key and its associated value from the store.
    ///
    /// Does nothing if the key doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to remove. Can be any type that converts to a string reference.
    ///
    /// # Errors
    ///
    /// Returns an error if the storage backend fails to remove the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use zep_kvs::prelude::*;
    ///
    /// let mut store = KeyValueStore::<scope::Ephemeral>::new()?;
    /// store.store("temp", "value")?;
    ///
    /// assert!(store.retrieve::<_, String>("temp")?.is_some());
    /// store.remove("temp")?;
    /// assert!(store.retrieve::<_, String>("temp")?.is_none());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn remove<K: AsRef<str>>(&mut self, key: K) -> Result<(), KvsError> {
        self.inner.remove(key.as_ref())
    }
}

/// Low-level interface for key-value storage backends.
///
/// This trait is implemented by platform-specific storage mechanisms
/// and handles the actual persistence of data. Most users should use
/// `KeyValueStore` instead of implementing this trait directly.
pub trait BackingStore {
    /// Returns all keys currently stored.
    ///
    /// # Errors
    ///
    /// Returns an error if the storage backend cannot be accessed.
    fn keys(&self) -> Result<Vec<String>, KvsError>;

    /// Stores raw bytes under the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to store the data under
    /// * `value` - The raw bytes to store
    ///
    /// # Errors
    ///
    /// Returns an error if the storage backend fails to write the data.
    fn store(&mut self, key: &str, value: &[u8]) -> Result<(), KvsError>;

    /// Retrieves raw bytes by key, if the key exists.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up
    ///
    /// # Returns
    ///
    /// Returns `Ok(Some(bytes))` if the key exists, `Ok(None)` if it doesn't,
    /// or an error if the storage backend fails.
    fn retrieve(&self, key: &str) -> Result<Option<Vec<u8>>, KvsError>;

    /// Removes a key and its associated data.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to remove
    ///
    /// # Errors
    ///
    /// Returns an error if the storage backend fails to remove the key.
    fn remove(&mut self, key: &str) -> Result<(), KvsError>;
}
