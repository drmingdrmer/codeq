use std::io::Error;
use std::io::Read;
use std::io::Write;
use std::marker::PhantomData;

use crate::codec::Decode;
use crate::codec::Encode;
use crate::config::CodeqConfig;
use crate::fixed_size::FixedSize;

/// A wrapper that appends a CRC32C checksum to the encoded data.
///
/// When data is encoded:
/// 1. The inner data is encoded first
/// 2. A checksum of the encoded data is calculated and appended
///
/// When data is decoded:
/// 1. The inner data is decoded first
/// 2. The checksum is verified against the decoded data, and an error is returned if they do not
///    match.
///
/// The generic parameter `C` specifies the checksum configuration to use for protecting the data.
///
/// Example:
#[cfg_attr(not(feature = "crc32fast"), doc = "```ignore")]
#[cfg_attr(feature = "crc32fast", doc = "```rust")]
/// # use codeq::{Encode, WithChecksum};
/// use codeq::config::Crc32fast;
///
/// let wc = WithChecksum::<Crc32fast, u64>::new(5);
/// let mut b = Vec::new();
/// let n = wc.encode(&mut b).unwrap();
/// assert_eq!(n, 16);
/// assert_eq!(
///     vec![
///         0, 0, 0, 0, 0, 0, 0, 5, // data
///         0, 0, 0, 0, 21, 72, 43, 230, // checksum
///     ],
///     b
/// );
/// ```
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
pub struct WithChecksum<C, T>
where C: CodeqConfig
{
    pub(crate) data: T,
    _p: PhantomData<C>,
}

impl<C, T> WithChecksum<C, T>
where C: CodeqConfig
{
    /// Creates a new wrapper around the given data.
    pub fn new(data: T) -> Self {
        Self {
            data,
            _p: Default::default(),
        }
    }

    /// Unwraps and returns the inner data
    pub fn into_inner(self) -> T {
        self.data
    }
}

impl<C, T> FixedSize for WithChecksum<C, T>
where
    C: CodeqConfig,
    T: FixedSize,
{
    fn encoded_size() -> usize {
        T::encoded_size() + 8
    }
}

impl<C, T> Encode for WithChecksum<C, T>
where
    C: CodeqConfig,
    T: Encode,
{
    fn encode<W: Write>(&self, mut w: W) -> Result<usize, Error> {
        let mut n = 0;
        let mut cw = C::new_writer(&mut w);

        n += self.data.encode(&mut cw)?;
        n += cw.write_checksum()?;

        Ok(n)
    }
}

impl<C, T> Decode for WithChecksum<C, T>
where
    C: CodeqConfig,
    T: Decode,
{
    fn decode<R: Read>(r: R) -> Result<Self, Error> {
        let mut cr = C::new_reader(r);

        let data = T::decode(&mut cr)?;
        cr.verify_checksum(|| "WithChecksum::decode()")?;

        let meta = Self {
            data,
            _p: Default::default(),
        };

        Ok(meta)
    }
}

#[cfg(feature = "crc32fast")]
#[cfg(test)]
mod tests_crc32fast {
    use crate::codec::Encode;
    use crate::config::CodeqConfig;
    use crate::config::Crc32fast;
    use crate::testing::test_codec;

    #[test]
    fn test_with_checksum_codec() -> anyhow::Result<()> {
        let wc = Crc32fast::wrap(5u64);
        let mut b = Vec::new();
        let n = wc.encode(&mut b)?;
        assert_eq!(n, b.len());

        assert_eq!(
            vec![
                0, 0, 0, 0, 0, 0, 0, 5, // data
                0, 0, 0, 0, 21, 72, 43, 230, // checksum
            ],
            b
        );

        test_codec(b.as_slice(), &wc)?;

        Ok(())
    }
}
