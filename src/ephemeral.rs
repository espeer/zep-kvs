//! In-memory storage implementation for ephemeral data.
//!
//! This module provides a HashMap-based storage backend that keeps
//! data in memory only. Data is lost when the store is dropped,
//! making it ideal for testing and temporary storage needs.

use std::collections::HashMap;

use crate::api::{BackingStore, Scope, scope::Ephemeral};
use crate::error::KvsError;

impl Scope for Ephemeral {
    type Store = EphemeralStore;

    fn new() -> Result<Self::Store, KvsError> {
        Ok(EphemeralStore::new())
    }
}

/// In-memory key-value store using a HashMap.
///
/// This store keeps all data in memory and provides fast access
/// to stored values. Data is not persisted and will be lost when
/// the store is dropped.
///
/// # Examples
///
/// ```
/// use zep_kvs::prelude::*;
///
/// let mut store = KeyValueStore::<scope::Ephemeral>::new()?;
/// store.store("test", "value")?;
/// let value: String = store.retrieve("test")?.unwrap();
/// assert_eq!(value, "value");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct EphemeralStore {
    store: HashMap<String, Vec<u8>>,
}

impl EphemeralStore {
    /// Creates a new empty ephemeral store.
    fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}

impl BackingStore for EphemeralStore {
    fn keys(&self) -> Result<Vec<String>, KvsError> {
        Ok(self.store.keys().map(|k| k.clone()).collect())
    }

    fn store(&mut self, key: &str, value: &[u8]) -> Result<(), KvsError> {
        self.store.insert(String::from(key), Vec::from(value));
        Ok(())
    }

    fn retrieve(&self, key: &str) -> Result<Option<Vec<u8>>, KvsError> {
        Ok(self.store.get(key).map(|value| value.clone()))
    }

    fn remove(&mut self, key: &str) -> Result<(), KvsError> {
        self.store.remove(key);
        Ok(())
    }
}
