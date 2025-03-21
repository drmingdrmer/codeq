use std::io;
use std::io::Error;
use std::io::Write;

/// A trait that can be encoded into an [`io::Write`] stream.
///
/// Implementing this trait allows types to be encoded into an [`io::Write`] stream,
/// which is useful for writing data to various destinations like files, buffers, and streams.
///
/// # Examples
/// ```rust
/// use codeq::Encode;
///
/// let data = "hello".to_string();
/// let mut buf = Vec::new();
/// data.encode(&mut buf).unwrap();
/// assert_eq!(buf, b"\x00\x00\x00\x05hello");
/// ```
pub trait Encode: Sized {
    fn encode<W: io::Write>(&self, w: W) -> Result<usize, io::Error>;

    /// Encodes the value into a new `Vec<u8>`.
    ///
    /// This method is sealed and cannot be implemented outside of the crate.
    fn encode_to_vec(&self) -> Result<Vec<u8>, Error>
    where Self: crate::sealed::Sealed {
        let mut buf = Vec::new();
        self.encode(&mut buf)?;
        Ok(buf)
    }
}

impl<T: Encode> Encode for &T {
    fn encode<W: Write>(&self, w: W) -> Result<usize, Error> {
        (*self).encode(w)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Error;
    use std::io::Write;

    use crate::codec::Encode;

    struct Foo;

    impl Encode for Foo {
        fn encode<W: Write>(&self, _w: W) -> Result<usize, Error> {
            Ok(3)
        }
    }

    #[test]
    fn test_encode_ref() {
        let foo = Foo;
        let n = Encode::encode(&foo, Vec::new()).unwrap();
        assert_eq!(n, 3);

        let n = Encode::encode(&&foo, Vec::new()).unwrap();
        assert_eq!(n, 3);
    }

    #[test]
    fn test_encode_to_vec() {
        let buf = 258u32.encode_to_vec().unwrap();
        assert_eq!(buf, vec![0, 0, 1, 2]);
    }
}
