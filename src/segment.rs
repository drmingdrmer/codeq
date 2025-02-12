use std::io::Error;
use std::io::Read;
use std::io::Write;
use std::marker::PhantomData;

use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;

use crate::ChecksumReader;
use crate::ChecksumWriter;
use crate::Decode;
use crate::Encode;
use crate::FixedSize;
use crate::Offset;
use crate::Size;
use crate::Span;

/// Represents a contiguous region in a file or buffer with an offset and size.
///
/// A segment is defined by:
/// - An offset: starting position in bytes
/// - A size: length in bytes
///
/// The generic parameter `T` can be used to associate additional type information
/// with the segment without affecting its binary representation.
#[derive(Debug, Clone, Copy)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Segment<T = ()> {
    /// Starting position of the segment in bytes
    pub offset: u64,

    /// Length of the segment in bytes
    pub size: u64,

    _p: PhantomData<T>,
}

impl<T> Segment<T> {
    /// Creates a new segment with the specified offset and size.
    ///
    /// # Arguments
    /// * `offset` - Starting position in bytes
    /// * `size` - Length in bytes
    pub fn new(offset: u64, size: u64) -> Self {
        Self {
            offset,
            size,
            _p: Default::default(),
        }
    }
}

impl<T> Span for Segment<T> {
    fn offset(&self) -> Offset {
        Offset(self.offset)
    }

    fn size(&self) -> Size {
        Size(self.size)
    }
}

impl FixedSize for Segment {
    /// Returns the fixed size of an encoded segment (24 bytes):
    /// - 8 bytes for offset
    /// - 8 bytes for size
    /// - 8 bytes for checksum
    fn encoded_size() -> usize {
        8 + 8 + 8
    }
}

impl Encode for Segment {
    fn encode<W: Write>(&self, mut w: W) -> Result<usize, Error> {
        let mut n = 0;

        let mut cw = ChecksumWriter::new(&mut w);

        cw.write_u64::<BigEndian>(self.offset)?;
        n += 8;

        cw.write_u64::<BigEndian>(self.size)?;
        n += 8;

        n += cw.write_checksum()?;

        Ok(n)
    }
}

impl Decode for Segment {
    fn decode<R: Read>(mut r: R) -> Result<Self, Error> {
        let mut cr = ChecksumReader::new(&mut r);

        let offset = cr.read_u64::<BigEndian>()?;
        let size = cr.read_u64::<BigEndian>()?;

        cr.verify_checksum(|| "Segment::decode()")?;

        Ok(Self {
            offset,
            size,
            _p: Default::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::segment::Segment;
    use crate::testing::test_codec;

    #[test]
    fn test_segment_codec() -> anyhow::Result<()> {
        let s = Segment {
            offset: 5,
            size: 10,
            _p: Default::default(),
        };

        let b = vec![
            0, 0, 0, 0, 0, 0, 0, 5, // offset
            0, 0, 0, 0, 0, 0, 0, 10, // size
            0, 0, 0, 0, 70, 249, 231, 4, // checksum
        ];

        test_codec(&b, &s)?;

        Ok(())
    }
}
