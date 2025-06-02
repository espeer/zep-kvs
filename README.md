![Zep Key-Value Store](logo.png "Zep Key-Value Store")

Zep Key-Value Store
===================

Zep-kvs is an elementary zero config cross platform key-value persistence library. Arbitrary
application data can be associated with named keys that can later be retrieved from an
operating system appropriate location.

## Features

- **Zero Configuration**: No setup required - just start storing data
- **Cross Platform**: Works on Linux, macOS, and Windows
- **Type Safe**: Generic API with automatic serialization/deserialization
- **Multiple Scopes**: Store data at user or machine level
- **Ephemeral Storage**: In-memory storage for testing
- **Platform Native**: Uses appropriate storage locations for each OS

## What Zep-KVS Is NOT

To set proper expectations, it's important to understand what zep-kvs is **not** designed for:

- **Not a Database**: Zep-kvs is not a replacement for PostgreSQL, SQLite, or other databases. It doesn't support queries, transactions, or relational data.

- **Not Distributed**: This is not a distributed key-value store like Redis, etcd, or Consul. It only stores data locally on a single machine.

- **Not High-Performance**: Zep-kvs prioritizes simplicity over performance. It's not optimized for high-throughput or low-latency operations.

- **Not a Cache**: While it can store temporary data, it's not designed as a caching layer with features like TTL, LRU eviction, or cache invalidation.

- **Not Network-Accessible**: Data is only accessible to the local application. There's no network interface or API for remote access.

- **Not for Large Data**: It's designed for small application settings and configuration data, not for storing large files or datasets.

- **Not ACID Compliant**: While individual operations are atomic, there's no support for multi-key transactions or complex consistency guarantees.

Zep-kvs is perfect for storing application preferences, user settings, small configuration data, and other simple key-value pairs that need to persist between application runs.

## Storage Locations

Zep-kvs automatically selects the appropriate storage location based on your operating system:

### Linux
- **User scope**: `$XDG_DATA_HOME` or `~/.local/share`
- **Machine scope**: `/var/lib`

### MacOS
- **User scope**: `~/Library/Application Support`
- **Machine scope**: `/Library/Application Support`

### Windows
- **User scope**: `HKEY_CURRENT_USER\Software`
- **Machine scope**: `HKEY_LOCAL_MACHINE\Software`

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
zep-kvs = "0.1.0"
```

### Basic Example

```rust
use zep_kvs::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a user-scoped key-value store
    let mut store = KeyValueStore::<scope::User>::new()?;

    // Store some data
    store.store("username", "alice")?;
    store.store("count", 42u32)?;

    // Retrieve data
    let username: String = store.retrieve("username")?.unwrap();
    let count: u32 = store.retrieve("count")?.unwrap();

    println!("Username: {}, Count: {}", username, count);

    // List all keys
    let keys = store.keys()?;
    println!("Stored keys: {:?}", keys);

    // Remove data
    store.remove("count")?;

    Ok(())
}
```

### Storage Scopes

Zep-kvs supports three different storage scopes:

#### User Scope
Store data specific to the current user:

```rust
use zep_kvs::prelude::*;

let mut store = KeyValueStore::<scope::User>::new()?;
store.store("user_preference", "dark_mode")?;
```

#### Machine Scope
Store data system-wide (requires appropriate permissions):

```rust
use zep_kvs::prelude::*;

let mut store = KeyValueStore::<scope::Machine>::new()?;
store.store("system_config", "production")?;
```

#### Ephemeral Scope
Store data in memory only (useful for testing):

```rust
use zep_kvs::prelude::*;

let mut store = KeyValueStore::<scope::Ephemeral>::new()?;
store.store("temp_data", "test_value")?;
// Data is lost when the store is dropped
```

### Data Types

Zep-kvs can store and retrieve various data types:

```rust
use zep_kvs::prelude::*;

let mut store = KeyValueStore::<scope::Ephemeral>::new()?;

// Strings
store.store("name", "John Doe")?;
let name: String = store.retrieve("name")?.unwrap();

// Numbers
store.store("age", 30u32)?;
let age: u32 = store.retrieve("age")?.unwrap();

// Binary data
let data = vec![1u8, 2u8, 3u8];
store.store("binary", data.as_slice())?;
let retrieved: Vec<u8> = store.retrieve("binary")?.unwrap();

// Check if a key exists
if let Some(value) = store.retrieve::<_, String>("optional_key")? {
    println!("Found: {}", value);
} else {
    println!("Key not found");
}
```

### Error Handling

Zep-kvs provides detailed error information:

```rust
use zep_kvs::prelude::*;

match KeyValueStore::<scope::User>::new() {
    Ok(mut store) => {
        match store.store("key", "value") {
            Ok(()) => println!("Stored successfully"),
            Err(e) => eprintln!("Storage error: {}", e),
        }
    }
    Err(e) => eprintln!("Failed to create store: {}", e),
}
```

## Platform-Specific Notes

### Linux
- User scope requires a home directory
- Machine scope requires write permissions to `/var/lib`
- Respects XDG Base Directory Specification

### MacOS
- Follows macOS conventions for application data storage
- Creates directories as needed in `Application Support`
- Requires appropriate permissions for machine scope

### Windows
- Uses Windows Registry for persistent storage
- User scope stores in `HKEY_CURRENT_USER`
- Machine scope requires administrator privileges

## Requirements

- Rust 1.75+ (uses `edition = "2024"`)

## License

Licensed under the BSD-2-Clause license. See LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
