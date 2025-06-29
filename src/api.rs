//! Core API for the Zep Key-Value Store library.
//!
//! This module provides the main interfaces for storing and retrieving data

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
    /// Data stored in this scope is available to all users of the system.
    pub struct Machine();

    /// User-specific storage that persists between program runs.
    ///
    /// Data is stored in the current user's profile.
    pub struct User();
}

/// A type-safe key-value store with configurable storage scope.
///
/// This is the main interface for storing and retrieving data. The generic
/// parameter `S` determines the storage scope (User, Machine, or Ephemeral).
pub struct KeyValueStore<S: Scope> {
    inner: S::Store,
}

impl<S: Scope> KeyValueStore<S> {
    /// Creates a new key-value store for the specified scope.
    ///
    /// # Errors
    ///
    /// Returns an error if the storage backend cannot be initialized.
    pub fn new() -> Result<Self, KvsError> {
        Ok(Self { inner: S::new()? })
    }

    /// Returns all keys currently stored in this store.
    ///
    /// # Errors
    ///
    /// Returns an error if the storage backend cannot be accessed.
    pub fn keys(&self) -> Result<Vec<String>, KvsError> {
        self.inner.keys()
    }

    /// Stores a value under the given key.
    ///
    /// If the key already exists, its value will be overwritten.
    /// The value can be any type that implements `OutBytes`, including
    /// strings and byte arrays.
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
