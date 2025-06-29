//! # Zep Key-Value Store
//!
//! A zero-configuration, cross-platform key-value persistence library that provides
//! simple storage for application data using operating system appropriate locations.
//!
//! ## Features
//!
//! - **Zero Configuration**: No setup required - just start storing data
//! - **Cross Platform**: Works on Linux, macOS, and Windows with platform-native storage
//! - **Type Safe**: Generic API with automatic serialization/deserialization
//! - **Multiple Scopes**: Store data at user, machine, or ephemeral (in-memory) levels
//! - **Platform Native**: Uses appropriate storage locations for each operating system
//!
//! ## Storage Locations
//!
//! The library automatically selects appropriate storage locations:
//!
//! ### Linux
//! - **User scope**: `$XDG_DATA_HOME/{app_name}` or `~/.local/share/{app_name}`
//! - **Machine scope**: `/var/lib/{app_name}`
//!
//! ### macOS
//! - **User scope**: `~/Library/Application Support/{app_name}`
//! - **Machine scope**: `/Library/Application Support/{app_name}`
//!
//! ### Windows
//! - **User scope**: `HKEY_CURRENT_USER\Software\{app_name}`
//! - **Machine scope**: `HKEY_LOCAL_MACHINE\Software\{app_name}`
//!
//! ## Quick Start
//!
//! ```rust
//! use zep_kvs::prelude::*;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a user-scoped store
//! let mut store = KeyValueStore::<scope::User>::new()?;
//!
//! // Store some data
//! store.store("username", "alice")?;
//! store.store("age", 42u32)?;
//!
//! // Retrieve data with type inference
//! let username: String = store.retrieve("username")?.unwrap();
//! let count: u32 = store.retrieve("age")?.unwrap();
//!
//! println!("User: {}, Count: {}", username, count);
//!
//! // List all stored keys
//! let keys = store.keys()?;
//! println!("Stored keys: {:?}", keys);
//!
//! // Remove data
//! store.remove("username")?;
//! store.remove("age")?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Storage Scopes
//!
//! Three storage scopes are available:
//!
//! - [`api::scope::User`] - User-specific data that persists between runs
//! - [`api::scope::Machine`] - System-wide data (requires elevated privileges)
//! - [`api::scope::Ephemeral`] - In-memory data for testing (not persistent)
//!
//! ## Data Types
//!
//! The store can handle various data types that implement the conversion traits:
//!
//! ```rust
//! use zep_kvs::prelude::*;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut store = KeyValueStore::<scope::Ephemeral>::new()?;
//!
//! // Strings
//! store.store("name", "John Doe")?;
//! let name: String = store.retrieve("name")?.unwrap();
//!
//! // Binary data
//! let data = vec![1u8, 2u8, 3u8];
//! store.store("binary", data.as_slice())?;
//! let retrieved: Vec<u8> = store.retrieve("binary")?.unwrap();
//!
//! // Check for missing keys
//! if let Some(value) = store.retrieve::<_, String>("missing_key")? {
//!     println!("Found: {}", value);
//! } else {
//!     println!("Key not found");
//! }
//! # Ok(())
//! # }
//! ```

pub mod api;
pub mod convert;
pub mod error;

mod ephemeral;

#[cfg(not(target_os = "windows"))]
mod directory;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

mod tests;

/// Re-exports of commonly used types and traits.
///
/// This module provides convenient access to the main API components
/// without needing to import them individually.
///
/// # Examples
///
/// ```rust
/// use zep_kvs::prelude::*;
///
/// let mut store = KeyValueStore::<scope::Ephemeral>::new()?;
/// store.store("key", "value")?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub mod prelude {
    pub use crate::api::{KeyValueStore, Scope, scope};
    pub use crate::convert::{InBytes, OutBytes};
}
