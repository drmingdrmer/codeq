use std::io;

/// A trait for types that can be decoded from an [`io::Read`] stream.
///
/// Implementing this trait allows types to be decoded from an [`io::Read`] stream,
/// which is useful for reading data from various sources like files, buffers, and streams.
///
/// # Examples
/// ```rust
/// use codeq::Decode;
///
/// let data = b"\x00\x00\x00\x05hello";
/// let decoded = String::decode(&data[..]).unwrap();
/// assert_eq!(decoded, "hello");
/// ```
pub trait Decode: Sized {
    fn decode<R: io::Read>(r: R) -> Result<Self, io::Error>;
}
