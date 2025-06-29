//! Tests for the Zep Key-Value Store library.
//!
//! This module contains integration tests that verify the functionality
//! of the key-value store across different scopes and data types.

#[cfg(test)]
use crate::prelude::*;

/// Test basic string storage and retrieval functionality.
///
/// Verifies that string values can be stored and retrieved correctly
/// using the ephemeral scope for fast, in-memory testing.
#[test]
fn can_store_and_retrieve_string() {
    let mut store = KeyValueStore::<scope::Ephemeral>::new().unwrap();
    store.store("abc", "def").unwrap();
    assert_eq!(store.retrieve("abc").unwrap(), Some(String::from("def")));
}

/// Test binary data storage and retrieval functionality.
///
/// Verifies that arbitrary binary data can be stored and retrieved
/// correctly as byte vectors.
#[test]
fn can_store_and_retrieve_bytes() {
    let mut store = KeyValueStore::<scope::Ephemeral>::new().unwrap();
    store.store("abc", [1u8, 2u8, 3u8].as_slice()).unwrap();
    assert_eq!(store.retrieve("abc").unwrap(), Some(vec![1u8, 2u8, 3u8]));
}

/// Test key enumeration functionality.
///
/// Verifies that all stored keys can be retrieved and that the
/// key listing functionality works correctly.
#[test]
fn can_retrieve_keys() {
    let mut machine = KeyValueStore::<scope::Ephemeral>::new().unwrap();
    machine.store("abc", "def").unwrap();
    machine.store("def", "hij").unwrap();
    machine.store("hij", "klm").unwrap();
    let keys = machine.keys().unwrap();
    assert!(keys.contains(&String::from("abc")));
    assert!(keys.contains(&String::from("def")));
    assert!(keys.contains(&String::from("hij")));
    assert_eq!(keys.len(), 3);
}

/// Test key removal functionality.
///
/// Verifies that keys can be removed from the store and that
/// other keys remain unaffected.
#[test]
fn can_remove_keys() {
    let mut store = KeyValueStore::<scope::Ephemeral>::new().unwrap();
    store.store("key1", "value1").unwrap();
    store.store("key2", "value2").unwrap();

    store.remove("key1").unwrap();
    assert_eq!(store.retrieve::<_, String>("key1").unwrap(), None);
    assert_eq!(
        store.retrieve("key2").unwrap(),
        Some(String::from("value2"))
    );
}

/// Test key overwriting functionality.
///
/// Verifies that existing keys can be overwritten with new values.
#[test]
fn can_overwrite_existing_keys() {
    let mut store = KeyValueStore::<scope::Ephemeral>::new().unwrap();
    store.store("key", "original").unwrap();
    store.store("key", "updated").unwrap();
    assert_eq!(
        store.retrieve("key").unwrap(),
        Some(String::from("updated"))
    );
}

/// Test handling of empty values.
///
/// Verifies that empty strings and byte arrays can be stored and retrieved.
#[test]
fn can_handle_empty_values() {
    let mut store = KeyValueStore::<scope::Ephemeral>::new().unwrap();
    store.store("empty_string", "").unwrap();
    store.store("empty_bytes", [].as_slice()).unwrap();

    assert_eq!(
        store.retrieve("empty_string").unwrap(),
        Some(String::from(""))
    );
    assert_eq!(
        store.retrieve("empty_bytes").unwrap(),
        Some(Vec::<u8>::new())
    );
}

/// Test handling of empty keys.
///
/// Verifies that empty string keys are supported.
#[test]
fn can_handle_empty_keys() {
    let mut store = KeyValueStore::<scope::Ephemeral>::new().unwrap();
    store.store("", "value").unwrap();
    assert_eq!(store.retrieve("").unwrap(), Some(String::from("value")));
}

/// Test handling of special characters in keys.
///
/// Verifies that keys with spaces, slashes, and other special characters work.
#[test]
fn can_handle_special_characters_in_keys() {
    let mut store = KeyValueStore::<scope::Ephemeral>::new().unwrap();
    let special_keys = [
        "key with spaces",
        "key/with/slashes",
        "key-with-dashes",
        "key.with.dots",
        "🔑emoji",
    ];

    for key in &special_keys {
        let value = format!("value for {}", key);
        store.store(*key, value.as_str()).unwrap();
    }

    for key in &special_keys {
        assert!(store.retrieve::<_, String>(*key).unwrap().is_some());
    }
}

/// Test handling of binary data with null bytes and edge cases.
///
/// Verifies that binary data with special byte values is handled correctly.
#[test]
fn can_handle_binary_data_with_nulls_and_special_bytes() {
    let mut store = KeyValueStore::<scope::Ephemeral>::new().unwrap();
    let binary_data = vec![0u8, 255u8, 127u8, 1u8, 0u8, 0u8, 42u8];

    store.store("binary", binary_data.as_slice()).unwrap();
    assert_eq!(store.retrieve("binary").unwrap(), Some(binary_data));
}

/// Test handling of large binary data.
///
/// Verifies that large amounts of binary data can be stored and retrieved.
#[test]
fn can_handle_large_binary_data() {
    let mut store = KeyValueStore::<scope::Ephemeral>::new().unwrap();
    let large_data: Vec<u8> = (0..10000).map(|i| (i % 256) as u8).collect();

    store.store("large", large_data.as_slice()).unwrap();
    assert_eq!(store.retrieve("large").unwrap(), Some(large_data));
}

/// Test behavior of empty store.
///
/// Verifies that a new store behaves correctly when empty.
#[test]
fn empty_store_has_sensible_behavior() {
    let store = KeyValueStore::<scope::Ephemeral>::new().unwrap();

    assert_eq!(store.keys().unwrap().len(), 0);
    assert_eq!(store.retrieve::<_, String>("nonexistent").unwrap(), None);
}

/// Test handling many keys for performance.
///
/// Verifies that the store can handle a large number of keys efficiently.
#[test]
fn can_handle_many_keys() {
    let mut store = KeyValueStore::<scope::Ephemeral>::new().unwrap();
    let num_keys = 1000;

    // Store many keys
    for i in 0..num_keys {
        let key = format!("key_{}", i);
        let value = format!("value_{}", i);
        store.store(key.as_str(), value.as_str()).unwrap();
    }

    // Verify all keys exist
    let keys = store.keys().unwrap();
    assert_eq!(keys.len(), num_keys);

    // Verify random access still works
    assert_eq!(
        store.retrieve("key_500").unwrap(),
        Some(String::from("value_500"))
    );
}

/// Test handling of Unicode strings.
///
/// Verifies that Unicode characters in values are handled correctly.
#[test]
fn can_handle_unicode_strings() {
    let mut store = KeyValueStore::<scope::Ephemeral>::new().unwrap();
    let unicode_strings = ["Hello, 世界!", "🚀🌟💫", "Ñoño", "Москва"];

    for (i, s) in unicode_strings.iter().enumerate() {
        store.store(format!("unicode_{}", i), *s).unwrap();
    }

    for (i, expected) in unicode_strings.iter().enumerate() {
        let retrieved: String = store.retrieve(format!("unicode_{}", i)).unwrap().unwrap();
        assert_eq!(retrieved, *expected);
    }
}

/// Test storage and retrieval of all primitive types.
///
/// Verifies that all Rust primitive types can be stored and retrieved
/// correctly through the key-value store.
#[test]
fn can_store_and_retrieve_primitive_types() {
    let mut store = KeyValueStore::<scope::Ephemeral>::new().unwrap();

    // Test boolean values
    store.store("bool_true", true).unwrap();
    store.store("bool_false", false).unwrap();
    assert_eq!(
        store.retrieve::<&str, bool>("bool_true").unwrap(),
        Some(true)
    );
    assert_eq!(
        store.retrieve::<&str, bool>("bool_false").unwrap(),
        Some(false)
    );

    // Test character values
    store.store("char_ascii", 'A').unwrap();
    store.store("char_unicode", '🚀').unwrap();
    assert_eq!(
        store.retrieve::<&str, char>("char_ascii").unwrap(),
        Some('A')
    );
    assert_eq!(
        store.retrieve::<&str, char>("char_unicode").unwrap(),
        Some('🚀')
    );

    // Test signed integers
    store.store("i8_val", 42i8).unwrap();
    store.store("i16_val", 1234i16).unwrap();
    store.store("i32_val", 123456i32).unwrap();
    store.store("i64_val", 1234567890i64).unwrap();
    store.store("i128_val", 123456789012345i128).unwrap();
    store.store("isize_val", 9999isize).unwrap();

    assert_eq!(store.retrieve::<&str, i8>("i8_val").unwrap(), Some(42i8));
    assert_eq!(
        store.retrieve::<&str, i16>("i16_val").unwrap(),
        Some(1234i16)
    );
    assert_eq!(
        store.retrieve::<&str, i32>("i32_val").unwrap(),
        Some(123456i32)
    );
    assert_eq!(
        store.retrieve::<&str, i64>("i64_val").unwrap(),
        Some(1234567890i64)
    );
    assert_eq!(
        store.retrieve::<&str, i128>("i128_val").unwrap(),
        Some(123456789012345i128)
    );
    assert_eq!(
        store.retrieve::<&str, isize>("isize_val").unwrap(),
        Some(9999isize)
    );

    // Test unsigned integers
    store.store("u8_val", 255u8).unwrap();
    store.store("u16_val", 65535u16).unwrap();
    store.store("u32_val", 4294967295u32).unwrap();
    store.store("u64_val", 18446744073709551615u64).unwrap();
    store
        .store("u128_val", 340282366920938463463374607431768211455u128)
        .unwrap();
    store.store("usize_val", 12345usize).unwrap();

    assert_eq!(store.retrieve::<&str, u8>("u8_val").unwrap(), Some(255u8));
    assert_eq!(
        store.retrieve::<&str, u16>("u16_val").unwrap(),
        Some(65535u16)
    );
    assert_eq!(
        store.retrieve::<&str, u32>("u32_val").unwrap(),
        Some(4294967295u32)
    );
    assert_eq!(
        store.retrieve::<&str, u64>("u64_val").unwrap(),
        Some(18446744073709551615u64)
    );
    assert_eq!(
        store.retrieve::<&str, u128>("u128_val").unwrap(),
        Some(340282366920938463463374607431768211455u128)
    );
    assert_eq!(
        store.retrieve::<&str, usize>("usize_val").unwrap(),
        Some(12345usize)
    );

    // Test floating-point values
    store.store("f32_val", 3.14159f32).unwrap();
    store.store("f64_val", 2.718281828459045f64).unwrap();

    assert_eq!(
        store.retrieve::<&str, f32>("f32_val").unwrap(),
        Some(3.14159f32)
    );
    assert_eq!(
        store.retrieve::<&str, f64>("f64_val").unwrap(),
        Some(2.718281828459045f64)
    );
}
