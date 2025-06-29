//! Type conversion traits for serializing and deserializing data.
//!
//! This module provides traits that allow the key-value store to work
//! with various data types by converting them to and from byte arrays
//! for storage.

use crate::error::KvsError;
use std::borrow::Cow;

/// Trait for types that can be converted to bytes for storage.
///
/// This trait is implemented for types that can be serialized
/// into a byte array for persistence in the key-value store.
pub trait OutBytes {
    /// Converts the value into bytes for storage.
    ///
    /// Returns a `Cow<[u8]>` which can be either borrowed (for types like `&str`)
    /// or owned (for primitive types that need conversion).
    ///
    /// # Errors
    ///
    /// Returns an error if the value cannot be serialized.
    fn out_bytes(&self) -> Result<Cow<[u8]>, KvsError>;
}

/// Trait for types that can be converted from bytes after retrieval.
///
/// This trait is implemented for types that can be deserialized
/// from a byte array when loading from the key-value store.
pub trait InBytes {
    /// Converts bytes back into the original type.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The byte array to deserialize from
    ///
    /// # Errors
    ///
    /// Returns an error if the bytes cannot be deserialized
    /// into the target type.
    fn in_bytes(bytes: &[u8]) -> Result<Self, KvsError>
    where
        Self: Sized;
}

/// Implementation for string slices.
impl OutBytes for &str {
    fn out_bytes(&self) -> Result<Cow<[u8]>, KvsError> {
        Ok(Cow::Borrowed(self.as_bytes()))
    }
}

/// Implementation for deserializing strings from UTF-8 bytes.
impl InBytes for String {
    fn in_bytes(bytes: &[u8]) -> Result<Self, KvsError> {
        Ok(String::from_utf8(Vec::from(bytes))?)
    }
}

/// Implementation for byte slices.
impl OutBytes for &[u8] {
    fn out_bytes(&self) -> Result<Cow<[u8]>, KvsError> {
        Ok(Cow::Borrowed(self))
    }
}

/// Implementation for deserializing byte vectors.
impl InBytes for Vec<u8> {
    fn in_bytes(bytes: &[u8]) -> Result<Self, KvsError> {
        Ok(Vec::from(bytes))
    }
}

// Boolean implementations
impl OutBytes for bool {
    fn out_bytes(&self) -> Result<Cow<[u8]>, KvsError> {
        Ok(Cow::Owned(vec![if *self { 1 } else { 0 }]))
    }
}

impl InBytes for bool {
    fn in_bytes(bytes: &[u8]) -> Result<Self, KvsError> {
        if bytes.len() != 1 {
            return Err(KvsError::SerializationError(
                "Invalid boolean byte length".to_string(),
            ));
        }
        match bytes[0] {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(KvsError::SerializationError(
                "Invalid boolean value".to_string(),
            )),
        }
    }
}

// Character implementations
impl OutBytes for char {
    fn out_bytes(&self) -> Result<Cow<[u8]>, KvsError> {
        let mut buf = [0u8; 4];
        let s = self.encode_utf8(&mut buf);
        Ok(Cow::Owned(s.as_bytes().to_vec()))
    }
}

impl InBytes for char {
    fn in_bytes(bytes: &[u8]) -> Result<Self, KvsError> {
        let s = std::str::from_utf8(bytes)
            .map_err(|_| KvsError::SerializationError("Invalid UTF-8 for char".to_string()))?;
        let mut chars = s.chars();
        let ch = chars
            .next()
            .ok_or_else(|| KvsError::SerializationError("Empty string for char".to_string()))?;
        if chars.next().is_some() {
            return Err(KvsError::SerializationError(
                "Multiple characters in char bytes".to_string(),
            ));
        }
        Ok(ch)
    }
}

// Signed integer implementations
impl OutBytes for i8 {
    fn out_bytes(&self) -> Result<Cow<[u8]>, KvsError> {
        Ok(Cow::Owned(self.to_be_bytes().to_vec()))
    }
}

impl InBytes for i8 {
    fn in_bytes(bytes: &[u8]) -> Result<Self, KvsError> {
        if bytes.len() != 1 {
            return Err(KvsError::SerializationError(
                "Invalid i8 byte length".to_string(),
            ));
        }
        Ok(i8::from_be_bytes([bytes[0]]))
    }
}

impl OutBytes for i16 {
    fn out_bytes(&self) -> Result<Cow<[u8]>, KvsError> {
        Ok(Cow::Owned(self.to_be_bytes().to_vec()))
    }
}

impl InBytes for i16 {
    fn in_bytes(bytes: &[u8]) -> Result<Self, KvsError> {
        if bytes.len() != 2 {
            return Err(KvsError::SerializationError(
                "Invalid i16 byte length".to_string(),
            ));
        }
        Ok(i16::from_be_bytes([bytes[0], bytes[1]]))
    }
}

impl OutBytes for i32 {
    fn out_bytes(&self) -> Result<Cow<[u8]>, KvsError> {
        Ok(Cow::Owned(self.to_be_bytes().to_vec()))
    }
}

impl InBytes for i32 {
    fn in_bytes(bytes: &[u8]) -> Result<Self, KvsError> {
        if bytes.len() != 4 {
            return Err(KvsError::SerializationError(
                "Invalid i32 byte length".to_string(),
            ));
        }
        let mut arr = [0u8; 4];
        arr.copy_from_slice(bytes);
        Ok(i32::from_be_bytes(arr))
    }
}

impl OutBytes for i64 {
    fn out_bytes(&self) -> Result<Cow<[u8]>, KvsError> {
        Ok(Cow::Owned(self.to_be_bytes().to_vec()))
    }
}

impl InBytes for i64 {
    fn in_bytes(bytes: &[u8]) -> Result<Self, KvsError> {
        if bytes.len() != 8 {
            return Err(KvsError::SerializationError(
                "Invalid i64 byte length".to_string(),
            ));
        }
        let mut arr = [0u8; 8];
        arr.copy_from_slice(bytes);
        Ok(i64::from_be_bytes(arr))
    }
}

impl OutBytes for i128 {
    fn out_bytes(&self) -> Result<Cow<[u8]>, KvsError> {
        Ok(Cow::Owned(self.to_be_bytes().to_vec()))
    }
}

impl InBytes for i128 {
    fn in_bytes(bytes: &[u8]) -> Result<Self, KvsError> {
        if bytes.len() != 16 {
            return Err(KvsError::SerializationError(
                "Invalid i128 byte length".to_string(),
            ));
        }
        let mut arr = [0u8; 16];
        arr.copy_from_slice(bytes);
        Ok(i128::from_be_bytes(arr))
    }
}

impl OutBytes for isize {
    fn out_bytes(&self) -> Result<Cow<[u8]>, KvsError> {
        Ok(Cow::Owned(self.to_be_bytes().to_vec()))
    }
}

impl InBytes for isize {
    fn in_bytes(bytes: &[u8]) -> Result<Self, KvsError> {
        if bytes.len() != std::mem::size_of::<isize>() {
            return Err(KvsError::SerializationError(
                "Invalid isize byte length".to_string(),
            ));
        }
        let mut arr = [0u8; 8];
        arr[..bytes.len()].copy_from_slice(bytes);
        Ok(isize::from_be_bytes(arr))
    }
}

// Unsigned integer implementations
impl OutBytes for u8 {
    fn out_bytes(&self) -> Result<Cow<[u8]>, KvsError> {
        Ok(Cow::Owned(self.to_be_bytes().to_vec()))
    }
}

impl InBytes for u8 {
    fn in_bytes(bytes: &[u8]) -> Result<Self, KvsError> {
        if bytes.len() != 1 {
            return Err(KvsError::SerializationError(
                "Invalid u8 byte length".to_string(),
            ));
        }
        Ok(u8::from_be_bytes([bytes[0]]))
    }
}

impl OutBytes for u16 {
    fn out_bytes(&self) -> Result<Cow<[u8]>, KvsError> {
        Ok(Cow::Owned(self.to_be_bytes().to_vec()))
    }
}

impl InBytes for u16 {
    fn in_bytes(bytes: &[u8]) -> Result<Self, KvsError> {
        if bytes.len() != 2 {
            return Err(KvsError::SerializationError(
                "Invalid u16 byte length".to_string(),
            ));
        }
        Ok(u16::from_be_bytes([bytes[0], bytes[1]]))
    }
}

impl OutBytes for u32 {
    fn out_bytes(&self) -> Result<Cow<[u8]>, KvsError> {
        Ok(Cow::Owned(self.to_be_bytes().to_vec()))
    }
}

impl InBytes for u32 {
    fn in_bytes(bytes: &[u8]) -> Result<Self, KvsError> {
        if bytes.len() != 4 {
            return Err(KvsError::SerializationError(
                "Invalid u32 byte length".to_string(),
            ));
        }
        let mut arr = [0u8; 4];
        arr.copy_from_slice(bytes);
        Ok(u32::from_be_bytes(arr))
    }
}

impl OutBytes for u64 {
    fn out_bytes(&self) -> Result<Cow<[u8]>, KvsError> {
        Ok(Cow::Owned(self.to_be_bytes().to_vec()))
    }
}

impl InBytes for u64 {
    fn in_bytes(bytes: &[u8]) -> Result<Self, KvsError> {
        if bytes.len() != 8 {
            return Err(KvsError::SerializationError(
                "Invalid u64 byte length".to_string(),
            ));
        }
        let mut arr = [0u8; 8];
        arr.copy_from_slice(bytes);
        Ok(u64::from_be_bytes(arr))
    }
}

impl OutBytes for u128 {
    fn out_bytes(&self) -> Result<Cow<[u8]>, KvsError> {
        Ok(Cow::Owned(self.to_be_bytes().to_vec()))
    }
}

impl InBytes for u128 {
    fn in_bytes(bytes: &[u8]) -> Result<Self, KvsError> {
        if bytes.len() != 16 {
            return Err(KvsError::SerializationError(
                "Invalid u128 byte length".to_string(),
            ));
        }
        let mut arr = [0u8; 16];
        arr.copy_from_slice(bytes);
        Ok(u128::from_be_bytes(arr))
    }
}

impl OutBytes for usize {
    fn out_bytes(&self) -> Result<Cow<[u8]>, KvsError> {
        Ok(Cow::Owned(self.to_be_bytes().to_vec()))
    }
}

impl InBytes for usize {
    fn in_bytes(bytes: &[u8]) -> Result<Self, KvsError> {
        if bytes.len() != std::mem::size_of::<usize>() {
            return Err(KvsError::SerializationError(
                "Invalid usize byte length".to_string(),
            ));
        }
        let mut arr = [0u8; 8];
        arr[..bytes.len()].copy_from_slice(bytes);
        Ok(usize::from_be_bytes(arr))
    }
}

// Floating-point implementations
impl OutBytes for f32 {
    fn out_bytes(&self) -> Result<Cow<[u8]>, KvsError> {
        Ok(Cow::Owned(self.to_be_bytes().to_vec()))
    }
}

impl InBytes for f32 {
    fn in_bytes(bytes: &[u8]) -> Result<Self, KvsError> {
        if bytes.len() != 4 {
            return Err(KvsError::SerializationError(
                "Invalid f32 byte length".to_string(),
            ));
        }
        let mut arr = [0u8; 4];
        arr.copy_from_slice(bytes);
        Ok(f32::from_be_bytes(arr))
    }
}

impl OutBytes for f64 {
    fn out_bytes(&self) -> Result<Cow<[u8]>, KvsError> {
        Ok(Cow::Owned(self.to_be_bytes().to_vec()))
    }
}

impl InBytes for f64 {
    fn in_bytes(bytes: &[u8]) -> Result<Self, KvsError> {
        if bytes.len() != 8 {
            return Err(KvsError::SerializationError(
                "Invalid f64 byte length".to_string(),
            ));
        }
        let mut arr = [0u8; 8];
        arr.copy_from_slice(bytes);
        Ok(f64::from_be_bytes(arr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool_conversion() {
        let true_val = true;
        let false_val = false;

        let true_bytes = true_val.out_bytes().unwrap();
        let false_bytes = false_val.out_bytes().unwrap();

        assert_eq!(true_bytes.as_ref(), &[1]);
        assert_eq!(false_bytes.as_ref(), &[0]);

        assert_eq!(bool::in_bytes(&[1]).unwrap(), true);
        assert_eq!(bool::in_bytes(&[0]).unwrap(), false);
    }

    #[test]
    fn test_integer_conversions() {
        let i32_val = 42i32;
        let u64_val = 1234567890u64;

        let i32_bytes = i32_val.out_bytes().unwrap();
        let u64_bytes = u64_val.out_bytes().unwrap();

        assert_eq!(i32::in_bytes(&i32_bytes).unwrap(), 42i32);
        assert_eq!(u64::in_bytes(&u64_bytes).unwrap(), 1234567890u64);
    }

    #[test]
    fn test_float_conversions() {
        let f32_val = 3.14f32;
        let f64_val = 2.718281828f64;

        let f32_bytes = f32_val.out_bytes().unwrap();
        let f64_bytes = f64_val.out_bytes().unwrap();

        assert_eq!(f32::in_bytes(&f32_bytes).unwrap(), 3.14f32);
        assert_eq!(f64::in_bytes(&f64_bytes).unwrap(), 2.718281828f64);
    }

    #[test]
    fn test_char_conversion() {
        let char_val = '🚀';
        let char_bytes = char_val.out_bytes().unwrap();

        assert_eq!(char::in_bytes(&char_bytes).unwrap(), '🚀');
    }

    #[test]
    fn test_cow_efficiency() {
        use std::borrow::Cow;

        // String slice uses borrowed data - no allocation
        let str_val = "hello world";
        let str_bytes = str_val.out_bytes().unwrap();
        assert!(matches!(str_bytes, Cow::Borrowed(_)));

        // Byte slice uses borrowed data - no allocation
        let byte_val: &[u8] = b"hello world";
        let byte_bytes = byte_val.out_bytes().unwrap();
        assert!(matches!(byte_bytes, Cow::Borrowed(_)));

        // Primitive types use owned data - efficient conversion
        let int_val = 42i32;
        let int_bytes = int_val.out_bytes().unwrap();
        assert!(matches!(int_bytes, Cow::Owned(_)));
        assert_eq!(int_bytes.len(), 4); // i32 is 4 bytes

        // Boolean uses owned data - single byte
        let bool_val = true;
        let bool_bytes = bool_val.out_bytes().unwrap();
        assert!(matches!(bool_bytes, Cow::Owned(_)));
        assert_eq!(bool_bytes.len(), 1); // bool is 1 byte
    }
}
