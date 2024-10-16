use std::any::type_name;
use std::fmt::Debug;
use std::io;
use std::mem::size_of;

use crate::codec::Codec;
use crate::FixedSize;

/// Test decoding from correct data and corrupted data.
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
