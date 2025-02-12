use std::io;

use byteorder::BigEndian;
use byteorder::WriteBytesExt;
use crc32fast::Hasher;

/// A writer that calculates CRC32 checksum while writing data.
///
/// This writer wraps any type implementing [`io::Write`] and transparently calculates
/// a CRC32 checksum of all data written through it. The checksum can be either:
/// - Retrieved using `finalize_checksum()`
/// - Written to the underlying writer using `write_checksum()`
///
/// Example:
/// ```rust
/// # use std::io::Write;
/// use codeq::ChecksumWriter;
///
/// let mut writer = ChecksumWriter::new(Vec::new());
/// writer.write_all(b"hello").unwrap();
/// let checksum = writer.finalize_checksum();
/// assert_eq!(checksum, crc32fast::hash(b"hello"));
/// ```
pub struct ChecksumWriter<W> {
    hasher: Hasher,
    inner: W,
}

impl<W> ChecksumWriter<W>
where W: io::Write
{
    /// Create a new [`ChecksumWriter`] that wraps the provided writer.
    pub fn new(inner: W) -> Self {
        Self {
            hasher: Hasher::new(),
            inner,
        }
    }

    /// Finalize the crc32 checksum and consume `self`.
    ///
    /// Return the checksum of all written data.
    #[allow(dead_code)]
    pub fn finalize_checksum(self) -> u32 {
        self.hasher.finalize()
    }

    /// Append the finalized crc32 checksum in the least significant 32 bits of a `u64` to the its
    /// inner writer, in BigEndian.
    ///
    /// Returns the number of bytes written.
    pub fn write_checksum(self) -> io::Result<usize> {
        let mut w = self.inner;
        let crc = self.hasher.finalize();
        w.write_u64::<BigEndian>(crc as u64)?;
        Ok(8)
    }
}

impl<W> io::Write for ChecksumWriter<W>
where W: io::Write
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.hasher.update(buf);
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

#[cfg(test)]
#[allow(clippy::redundant_clone)]
mod tests {
    use std::io::Write;

    #[test]
    fn test_checksum_writer() -> anyhow::Result<()> {
        let mut b = Vec::new();

        // empty buffer
        {
            let w = super::ChecksumWriter::new(&mut b);
            let crc = w.finalize_checksum();

            assert_eq!(crc32fast::hash(b""), crc);
        }

        // write something
        {
            let mut n = 0;

            let mut w = super::ChecksumWriter::new(&mut b);

            n += w.write(b"foo")?;
            n += w.write(b"bar")?;
            assert_eq!(n, 6);

            let crc = w.finalize_checksum();

            assert_eq!(crc32fast::hash(b"foobar"), crc);
            assert_eq!(b"foobar", b.as_slice());
        }

        Ok(())
    }
    #[test]
    fn test_checksum_writer_finalize_to_inner() -> anyhow::Result<()> {
        let mut b = Vec::new();

        let mut n = 0;
        let mut w = super::ChecksumWriter::new(&mut b);
        n += w.write(b"foo")?;
        n += w.write(b"bar")?;
        n += w.write_checksum()?;

        assert_eq!(n, 14);
        assert_eq!(
            vec![102, 111, 111, 98, 97, 114, 0, 0, 0, 0, 158, 246, 31, 149],
            b
        );

        Ok(())
    }
}
