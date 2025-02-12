use std::io::Error;
use std::io::Read;
use std::io::Write;

use crate::checksum_reader::ChecksumReader;
use crate::checksum_writer::ChecksumWriter;
use crate::codec::Decode;
use crate::codec::Encode;
use crate::fixed_size::FixedSize;

/// A wrapper that appends a CRC32C checksum to the encoded data.
///
/// An 8-byte checksum is appended to the end of the encoded data.
/// - When encoding, the checksum is appended to the data after the inner `T` is encoded.
/// - When decoding, the checksum is verified against the decoded data after the inner `T` is
///   decoded, and an error is returned if they do not match.
///
/// Example:
/// ```rust
/// # use codeq::{Encode, WithChecksum};
///
/// let wc = WithChecksum::<u64>::new(5);
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
pub struct WithChecksum<T> {
    pub(crate) data: T,
}

impl<T> WithChecksum<T> {
    /// Creates a new wrapper around the given data.
    pub fn new(data: T) -> Self {
        Self { data }
    }

    /// Unwraps and returns the inner data
    pub fn into_inner(self) -> T {
        self.data
    }
}

impl<T: FixedSize> FixedSize for WithChecksum<T> {
    fn encoded_size() -> usize {
        T::encoded_size() + 8
    }
}

impl<T> Encode for WithChecksum<T>
where T: Encode
{
    fn encode<W: Write>(&self, mut w: W) -> Result<usize, Error> {
        let mut n = 0;
        let mut cw = ChecksumWriter::new(&mut w);

        n += self.data.encode(&mut cw)?;
        n += cw.write_checksum()?;

        Ok(n)
    }
}

impl<T> Decode for WithChecksum<T>
where T: Decode
{
    fn decode<R: Read>(r: R) -> Result<Self, Error> {
        let mut cr = ChecksumReader::new(r);

        let data = T::decode(&mut cr)?;
        cr.verify_checksum(|| "WithChecksum::decode()")?;

        let meta = Self { data };

        Ok(meta)
    }
}

#[cfg(test)]
mod tests {
    use crate::codec::Encode;
    use crate::testing::test_codec;
    use crate::with_checksum::WithChecksum;

    #[test]
    fn test_with_checksum_codec() -> anyhow::Result<()> {
        let wc = WithChecksum::<u64>::new(5);
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
