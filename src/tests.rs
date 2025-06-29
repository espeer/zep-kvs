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
/// Test user scope storage functionality.
///
/// Verifies that the user scope can store, retrieve, and remove data
/// correctly. Also tests that non-existent keys return None.
#[test]
fn can_store_user_scope() {
    let mut user = KeyValueStore::<scope::User>::new().unwrap();
    user.store("foo", "bar").unwrap();
    assert!(user.keys().unwrap().contains(&String::from("foo")));
    assert_eq!(user.retrieve("foo").unwrap(), Some("bar".to_owned()));
    assert_eq!(user.retrieve::<_, String>("baz").unwrap(), None);
    user.remove("foo").unwrap();
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

/// Verifies that data stored in user scope persists when the store
/// is dropped and recreated, confirming backing store persistence.
#[test]
fn user_scope_persists_across_instances() {
    let test_key = "user_persistence_test";
    let test_value = "persistent_data";

    // Store data in first instance
    {
        let mut store = KeyValueStore::<scope::User>::new().unwrap();
        store.store(test_key, test_value).unwrap();
    }

    // Verify data persists in second instance
    {
        let store = KeyValueStore::<scope::User>::new().unwrap();
        assert_eq!(
            store.retrieve(test_key).unwrap(),
            Some(String::from(test_value))
        );
    }

    // Clean up
    {
        let mut store = KeyValueStore::<scope::User>::new().unwrap();
        store.remove(test_key).unwrap();
    }
}

/// Verifies that user scope can handle all primitive types
#[test]
fn user_scope_handles_primitive_types() {
    let mut store = KeyValueStore::<scope::User>::new().unwrap();

    // Test a representative sample of primitive types
    store.store("user_bool", true).unwrap();
    store.store("user_i32", 42i32).unwrap();
    store.store("user_u64", 1234567890u64).unwrap();
    store.store("user_f64", 3.14159f64).unwrap();
    store.store("user_char", '🔥').unwrap();

    assert_eq!(
        store.retrieve::<&str, bool>("user_bool").unwrap(),
        Some(true)
    );
    assert_eq!(
        store.retrieve::<&str, i32>("user_i32").unwrap(),
        Some(42i32)
    );
    assert_eq!(
        store.retrieve::<&str, u64>("user_u64").unwrap(),
        Some(1234567890u64)
    );
    assert_eq!(
        store.retrieve::<&str, f64>("user_f64").unwrap(),
        Some(3.14159f64)
    );
    assert_eq!(
        store.retrieve::<&str, char>("user_char").unwrap(),
        Some('🔥')
    );

    // Clean up
    store.remove("user_bool").unwrap();
    store.remove("user_i32").unwrap();
    store.remove("user_u64").unwrap();
    store.remove("user_f64").unwrap();
    store.remove("user_char").unwrap();
}

/// Verifies that user scope can handle binary data, empty values,
/// and special characters in keys.
#[test]
fn user_scope_handles_binary_and_edge_cases() {
    let mut store = KeyValueStore::<scope::User>::new().unwrap();

    // Test binary data with null bytes
    let binary_data = vec![0u8, 255u8, 127u8, 1u8, 0u8, 0u8, 42u8];
    store.store("user_binary", binary_data.as_slice()).unwrap();
    assert_eq!(store.retrieve("user_binary").unwrap(), Some(binary_data));

    // Test empty values
    store.store("user_empty", "").unwrap();
    assert_eq!(
        store.retrieve("user_empty").unwrap(),
        Some(String::from(""))
    );

    // Test special characters in keys
    store.store("user-key.test", "special_key_value").unwrap();
    assert_eq!(
        store.retrieve("user-key.test").unwrap(),
        Some(String::from("special_key_value"))
    );

    // Clean up
    store.remove("user_binary").unwrap();
    store.remove("user_empty").unwrap();
    store.remove("user-key.test").unwrap();
}

/// Test user scope key operations (overwrite, remove, list).
#[test]
fn user_scope_key_operations() {
    let mut store = KeyValueStore::<scope::User>::new().unwrap();

    // Test overwriting
    store.store("user_overwrite", "original").unwrap();
    store.store("user_overwrite", "updated").unwrap();
    assert_eq!(
        store.retrieve("user_overwrite").unwrap(),
        Some(String::from("updated"))
    );

    // Test key listing
    store.store("user_key1", "value1").unwrap();
    store.store("user_key2", "value2").unwrap();
    let keys = store.keys().unwrap();
    assert!(keys.contains(&String::from("user_overwrite")));
    assert!(keys.contains(&String::from("user_key1")));
    assert!(keys.contains(&String::from("user_key2")));

    // Test removal
    store.remove("user_key1").unwrap();
    let keys_after_remove = store.keys().unwrap();
    assert!(!keys_after_remove.contains(&String::from("user_key1")));
    assert!(keys_after_remove.contains(&String::from("user_key2")));

    // Test retrieval after removal
    assert_eq!(store.retrieve::<_, String>("user_key1").unwrap(), None);
    assert_eq!(
        store.retrieve("user_key2").unwrap(),
        Some(String::from("value2"))
    );

    // Clean up
    store.remove("user_overwrite").unwrap();
    store.remove("user_key2").unwrap();
}

/// Verifies that user scope properly handles Unicode data
#[test]
fn user_scope_handles_unicode() {
    let mut store = KeyValueStore::<scope::User>::new().unwrap();

    let unicode_strings = [
        "Hello, 世界!",
        "🚀🌟💫",
        "Ñoño",
        "Москва",
        "مرحبا",
        "こんにちは",
    ];

    // Store all Unicode strings
    for (i, s) in unicode_strings.iter().enumerate() {
        let key = format!("user_unicode_{}", i);
        store.store(&key, *s).unwrap();
    }

    // Verify all Unicode strings are retrieved correctly
    for (i, expected) in unicode_strings.iter().enumerate() {
        let key = format!("user_unicode_{}", i);
        let retrieved: String = store.retrieve(&key).unwrap().unwrap();
        assert_eq!(retrieved, *expected);
    }

    // Clean up
    for i in 0..unicode_strings.len() {
        let key = format!("user_unicode_{}", i);
        store.remove(&key).unwrap();
    }
}

/// Verifies that user scope maintains data consistency across
/// multiple store, retrieve, and remove operations.
#[test]
fn user_scope_data_consistency() {
    let mut store = KeyValueStore::<scope::User>::new().unwrap();

    // Perform multiple operations to test consistency
    let operations = [
        ("consistency_1", "value_1"),
        ("consistency_2", "value_2"),
        ("consistency_3", "value_3"),
    ];

    // Store all values
    for (key, value) in &operations {
        store.store(*key, *value).unwrap();
    }

    // Verify all values are stored
    for (key, expected_value) in &operations {
        assert_eq!(
            store.retrieve(*key).unwrap(),
            Some(String::from(*expected_value))
        );
    }

    // Update some values
    store.store("consistency_1", "updated_value_1").unwrap();
    store.store("consistency_3", "updated_value_3").unwrap();

    // Verify updates
    assert_eq!(
        store.retrieve("consistency_1").unwrap(),
        Some(String::from("updated_value_1"))
    );
    assert_eq!(
        store.retrieve("consistency_2").unwrap(),
        Some(String::from("value_2"))
    );
    assert_eq!(
        store.retrieve("consistency_3").unwrap(),
        Some(String::from("updated_value_3"))
    );

    // Remove one key
    store.remove("consistency_2").unwrap();

    // Verify removal
    assert_eq!(store.retrieve::<_, String>("consistency_2").unwrap(), None);
    assert_eq!(
        store.retrieve("consistency_1").unwrap(),
        Some(String::from("updated_value_1"))
    );
    assert_eq!(
        store.retrieve("consistency_3").unwrap(),
        Some(String::from("updated_value_3"))
    );

    // Clean up
    store.remove("consistency_1").unwrap();
    store.remove("consistency_3").unwrap();
}

/// Verifies that Ephemeral and User scopes maintain separate data
/// and that operations on one scope don't affect the other.
#[test]
fn storage_scopes_are_independent() {
    let mut ephemeral_store = KeyValueStore::<scope::Ephemeral>::new().unwrap();
    let mut user_store = KeyValueStore::<scope::User>::new().unwrap();

    // Store same key with different values in each scope
    ephemeral_store
        .store("scope_test", "ephemeral_value")
        .unwrap();
    user_store.store("scope_test", "user_value").unwrap();

    // Verify each scope has its own value
    assert_eq!(
        ephemeral_store.retrieve("scope_test").unwrap(),
        Some(String::from("ephemeral_value"))
    );
    assert_eq!(
        user_store.retrieve("scope_test").unwrap(),
        Some(String::from("user_value"))
    );

    // Store additional keys in each scope
    ephemeral_store
        .store("ephemeral_only", "ephemeral_data")
        .unwrap();
    user_store.store("user_only", "user_data").unwrap();

    // Verify scope isolation - each scope only sees its own keys
    let ephemeral_keys = ephemeral_store.keys().unwrap();
    let user_keys = user_store.keys().unwrap();

    assert!(ephemeral_keys.contains(&String::from("scope_test")));
    assert!(ephemeral_keys.contains(&String::from("ephemeral_only")));
    assert!(!ephemeral_keys.contains(&String::from("user_only")));

    assert!(user_keys.contains(&String::from("scope_test")));
    assert!(user_keys.contains(&String::from("user_only")));
    assert!(!user_keys.contains(&String::from("ephemeral_only")));

    // Verify cross-scope key retrieval returns None
    assert_eq!(
        ephemeral_store.retrieve::<_, String>("user_only").unwrap(),
        None
    );
    assert_eq!(
        user_store.retrieve::<_, String>("ephemeral_only").unwrap(),
        None
    );

    // Remove key from one scope, verify it doesn't affect the other
    ephemeral_store.remove("scope_test").unwrap();
    assert_eq!(
        ephemeral_store.retrieve::<_, String>("scope_test").unwrap(),
        None
    );
    assert_eq!(
        user_store.retrieve("scope_test").unwrap(),
        Some(String::from("user_value"))
    );

    // Clean up user scope (ephemeral scope cleans up automatically)
    user_store.remove("scope_test").unwrap();
    user_store.remove("user_only").unwrap();
}
