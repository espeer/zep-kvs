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
    fn out_bytes(&self) -> Result<Cow<'_, [u8]>, KvsError>;
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
    fn out_bytes(&self) -> Result<Cow<'_, [u8]>, KvsError> {
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
    fn out_bytes(&self) -> Result<Cow<'_, [u8]>, KvsError> {
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
    fn out_bytes(&self) -> Result<Cow<'_, [u8]>, KvsError> {
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
    fn out_bytes(&self) -> Result<Cow<'_, [u8]>, KvsError> {
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
    fn out_bytes(&self) -> Result<Cow<'_, [u8]>, KvsError> {
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
    fn out_bytes(&self) -> Result<Cow<'_, [u8]>, KvsError> {
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
    fn out_bytes(&self) -> Result<Cow<'_, [u8]>, KvsError> {
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
    fn out_bytes(&self) -> Result<Cow<'_, [u8]>, KvsError> {
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
    fn out_bytes(&self) -> Result<Cow<'_, [u8]>, KvsError> {
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
    fn out_bytes(&self) -> Result<Cow<'_, [u8]>, KvsError> {
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
        let mut arr = [0u8; std::mem::size_of::<isize>()];
        arr.copy_from_slice(bytes);
        Ok(isize::from_be_bytes(arr))
    }
}

// Unsigned integer implementations
impl OutBytes for u8 {
    fn out_bytes(&self) -> Result<Cow<'_, [u8]>, KvsError> {
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
    fn out_bytes(&self) -> Result<Cow<'_, [u8]>, KvsError> {
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
    fn out_bytes(&self) -> Result<Cow<'_, [u8]>, KvsError> {
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
    fn out_bytes(&self) -> Result<Cow<'_, [u8]>, KvsError> {
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
    fn out_bytes(&self) -> Result<Cow<'_, [u8]>, KvsError> {
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
    fn out_bytes(&self) -> Result<Cow<'_, [u8]>, KvsError> {
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
        let mut arr = [0u8; std::mem::size_of::<usize>()];
        arr.copy_from_slice(bytes);
        Ok(usize::from_be_bytes(arr))
    }
}

// Floating-point implementations
impl OutBytes for f32 {
    fn out_bytes(&self) -> Result<Cow<'_, [u8]>, KvsError> {
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
    fn out_bytes(&self) -> Result<Cow<'_, [u8]>, KvsError> {
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

// Fixed-size u8 array implementations using macro
macro_rules! impl_fixed_u8_array {
    ($($n:expr),*) => {
        $(
            impl OutBytes for [u8; $n] {
                fn out_bytes(&self) -> Result<Cow<'_, [u8]>, KvsError> {
                    // Use borrowed data for efficiency - no need to allocate a new vector
                    Ok(Cow::Borrowed(self))
                }
            }

            impl InBytes for [u8; $n] {
                fn in_bytes(bytes: &[u8]) -> Result<Self, KvsError> {
                    if bytes.len() != $n {
                        return Err(KvsError::SerializationError(
                            format!("Invalid [u8; {}] byte length: expected {} bytes, got {}",
                                $n, $n, bytes.len()),
                        ));
                    }

                    let mut arr = [0u8; $n];
                    arr.copy_from_slice(bytes);
                    Ok(arr)
                }
            }
        )*
    };
}

// Implement for arrays from [u8; 1] to [u8; 64]
impl_fixed_u8_array!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
);

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
    fn test_fixed_array_conversions() {
        // Test [u8; 1]
        let arr1: [u8; 1] = [42];
        let arr1_bytes = arr1.out_bytes().unwrap();
        assert_eq!(<[u8; 1]>::in_bytes(&arr1_bytes).unwrap(), [42]);

        // Test [u8; 4]
        let arr4: [u8; 4] = [1, 2, 3, 4];
        let arr4_bytes = arr4.out_bytes().unwrap();
        assert_eq!(<[u8; 4]>::in_bytes(&arr4_bytes).unwrap(), [1, 2, 3, 4]);

        // Test [u8; 16]
        let arr16: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let arr16_bytes = arr16.out_bytes().unwrap();
        assert_eq!(<[u8; 16]>::in_bytes(&arr16_bytes).unwrap(), arr16);

        // Test [u8; 32]
        let arr32: [u8; 32] = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32,
        ];
        let arr32_bytes = arr32.out_bytes().unwrap();
        assert_eq!(<[u8; 32]>::in_bytes(&arr32_bytes).unwrap(), arr32);

        // Test [u8; 64]
        let arr64: [u8; 64] = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64,
        ];
        let arr64_bytes = arr64.out_bytes().unwrap();
        assert_eq!(<[u8; 64]>::in_bytes(&arr64_bytes).unwrap(), arr64);
    }

    #[test]
    fn test_fixed_array_error_handling() {
        // Test with incorrect byte length
        let result = <[u8; 4]>::in_bytes(&[1, 2, 3]);
        assert!(result.is_err());
        if let Err(KvsError::SerializationError(msg)) = result {
            assert!(msg.contains("Invalid [u8; 4] byte length"));
            assert!(msg.contains("expected 4 bytes, got 3"));
        } else {
            panic!("Expected SerializationError");
        }

        // Test passing array directly
        let src_arr = [1u8, 2, 3, 4];
        let result = <[u8; 4]>::in_bytes(&src_arr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), [1, 2, 3, 4]);
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

        // Fixed-size arrays use borrowed data for efficiency
        let arr_val: [u8; 4] = [1, 2, 3, 4];
        let arr_bytes = arr_val.out_bytes().unwrap();
        assert!(matches!(arr_bytes, Cow::Borrowed(_)));
        assert_eq!(arr_bytes.len(), 4);
    }
}
