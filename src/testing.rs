//! Test utilities for codec implementations.

use std::any::type_name;
use std::fmt::Debug;
use std::io;
use std::mem::size_of;

use crate::codec::Codec;
use crate::FixedSize;

/// Comprehensively tests a codec implementation with both valid and corrupted data.
///
/// This function performs several checks:
/// 1. Encodes the value and verifies it matches the expected bytes
/// 2. Decodes the encoded bytes and verifies it matches the original value
/// 3. Tests error handling by corrupting each byte and ensuring decode fails
///
/// # Arguments
/// * `encoded_bytes` - The expected encoded representation
/// * `v` - The value to test encoding/decoding
///
/// # Returns
/// `Ok(())` if all tests pass, `io::Error` if any encoding/decoding operation fails
pub fn test_codec<D: Codec + PartialEq + Debug>(
    encoded_bytes: &[u8],
    v: &D,
) -> Result<(), io::Error> {
    // convert `correct` to string if possible
    let correct_str = String::from_utf8_lossy(encoded_bytes);
    println!("correct data: {}", correct_str);

    let mes = format!("Type: {} encoded data: {:?}", type_name::<D>(), correct_str);

    // Test encoding
    {
        let mut b = Vec::new();
        let n = v.encode(&mut b)?;
        assert_eq!(n, b.len(), "output len, {}", &mes);
        assert_eq!(b, encoded_bytes, "output data, {}", &mes);
    }

    // Assert the input is correct

    {
        let b = encoded_bytes.to_vec();
        let decoded = D::decode(&mut b.as_slice())?;
        assert_eq!(v, &decoded, "decode, {}", &mes);
    }

    // Assert corrupted data returns error
    for i in 0..encoded_bytes.len() {
        let mut corrupted = encoded_bytes.to_vec();
        corrupted[i] = corrupted[i].wrapping_add(1);

        let res = D::decode(&mut corrupted.as_slice());
        assert!(
            res.is_err(),
            "change {}-th byte for type {}; the correct encoded data is: {}",
            i,
            type_name::<D>(),
            correct_str
        );
    }

    Ok(())
}

/// Tests integer codec implementations for fixed-size types.
///
/// Verifies that:
/// 1. The encoded size matches the type's size in memory
/// 2. The value can be encoded and decoded correctly
/// 3. The encoded length matches the expected size
///
/// # Arguments
/// * `v` - The integer value to test
///
/// # Returns
/// `Ok(())` if all tests pass, or an error if any check fails
pub fn test_int_coded<T: Codec + FixedSize + PartialEq + Debug>(v: T) -> anyhow::Result<()> {
    let size = size_of::<T>();

    assert_eq!(T::encoded_size(), size);

    let mut buf = Vec::new();
    let n = v.encode(&mut buf)?;
    assert_eq!(n, buf.len());

    let b = T::decode(&mut buf.as_slice())?;
    assert_eq!(v, b);

    Ok(())
}
