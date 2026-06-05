use std::hash::Hasher;
use std::io;

use byteorder::BigEndian;
use byteorder::WriteBytesExt;

use crate::config::CodeqConfig;

/// A writer that calculates CRC32 checksum while writing data.
///
/// This writer wraps any type implementing [`io::Write`] and transparently calculates
/// a CRC32 checksum of all data written through it. The checksum can be either:
/// - Retrieved using `finalize_checksum()`
/// - Written to the underlying writer using `write_checksum()`
/// - Written with `finalize()`, which returns the total bytes written
///
/// Example:
#[cfg_attr(not(feature = "crc32fast"), doc = "```ignore")]
#[cfg_attr(feature = "crc32fast", doc = "```rust")]
/// # use std::io::Write;
/// use codeq::ChecksumWriter;
/// use codeq::config::Crc32fast;
///
/// let mut writer = ChecksumWriter::<Crc32fast,_>::new(Vec::new());
/// writer.write_all(b"hello").unwrap();
/// let checksum = writer.finalize_checksum();
/// assert_eq!(checksum, crc32fast::hash(b"hello") as u64);
/// ```
/// 
/// Create a new writer with [`ChecksumWriter::new`] or [`CodeqConfig::new_writer`], for example:
/// ```ignore
/// let writer = Crc32fast::new_writer(Vec::new());
/// ```
pub struct ChecksumWriter<C, W>
where C: CodeqConfig
{
    hasher: C::Hasher,
    inner: W,
    written: usize,
}

impl<C, W> ChecksumWriter<C, W>
where
    C: CodeqConfig,
    W: io::Write,
{
    /// Create a new [`ChecksumWriter`] that wraps the provided writer.
    pub fn new(inner: W) -> Self {
        Self {
            hasher: Default::default(),
            inner,
            written: 0,
        }
    }

    /// Finalize the crc32 checksum and consume `self`.
    ///
    /// Return the checksum of all written data.
    #[allow(dead_code)]
    pub fn finalize_checksum(self) -> u64 {
        self.hasher.finish()
    }

    /// Append the finalized crc32 checksum in the least significant 32 bits of a `u64` to the its
    /// inner writer, in BigEndian.
    ///
    /// Returns the number of bytes written.
    pub fn write_checksum(self) -> io::Result<usize> {
        let mut w = self.inner;
        let crc = self.hasher.finish();
        w.write_u64::<BigEndian>(crc)?;
        Ok(8)
    }

    /// Append the finalized checksum to the inner writer and return the total
    /// bytes written, including the checksum.
    pub fn finalize(self) -> io::Result<usize> {
        let mut w = self.inner;
        let crc = self.hasher.finish();
        w.write_u64::<BigEndian>(crc)?;
        Ok(self.written + 8)
    }
}

impl<C, W> io::Write for ChecksumWriter<C, W>
where
    C: CodeqConfig,
    W: io::Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let written = self.inner.write(buf)?;
        self.hasher.write(&buf[..written]);
        self.written += written;
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

#[cfg(feature = "crc32fast")]
#[cfg(test)]
#[allow(clippy::redundant_clone)]
mod tests_crc32fast {
    use std::io;
    use std::io::Write;

    use crate::config::CodeqConfig;
    use crate::config::Crc32fast;

    #[test]
    fn test_checksum_writer() -> anyhow::Result<()> {
        let mut b = Vec::new();

        // empty buffer
        {
            let w = Crc32fast::new_writer(&mut b);
            let crc = w.finalize_checksum();

            assert_eq!(crc32fast::hash(b"") as u64, crc);
        }

        // write something
        {
            let mut n = 0;

            let mut w = Crc32fast::new_writer(&mut b);

            n += w.write(b"foo")?;
            n += w.write(b"bar")?;
            assert_eq!(n, 6);

            let crc = w.finalize_checksum();

            assert_eq!(crc32fast::hash(b"foobar") as u64, crc);
            assert_eq!(b"foobar", b.as_slice());
        }

        Ok(())
    }

    #[test]
    fn test_checksum_writer_finalize_to_inner() -> anyhow::Result<()> {
        let mut b = Vec::new();

        let mut n = 0;
        let mut w = Crc32fast::new_writer(&mut b);
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

    #[test]
    fn test_checksum_writer_finalize_returns_total_written() -> anyhow::Result<()> {
        let mut b = Vec::new();

        let mut w = Crc32fast::new_writer(&mut b);
        w.write_all(b"foobar")?;
        let n = w.finalize()?;

        assert_eq!(n, 14);
        assert_eq!(
            vec![102, 111, 111, 98, 97, 114, 0, 0, 0, 0, 158, 246, 31, 149],
            b
        );

        Ok(())
    }

    struct OneByteWriter<'a>(&'a mut Vec<u8>);

    impl Write for OneByteWriter<'_> {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if buf.is_empty() {
                return Ok(0);
            }
            self.0.push(buf[0]);
            Ok(1)
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_checksum_writer_counts_only_accepted_bytes() -> anyhow::Result<()> {
        let mut b = Vec::new();

        let mut w = Crc32fast::new_writer(OneByteWriter(&mut b));
        assert_eq!(w.write(b"foobar")?, 1);
        assert_eq!(w.finalize()?, 9);

        assert_eq!(b[0], b'f');
        assert_eq!(&b[1..], &(crc32fast::hash(b"f") as u64).to_be_bytes());

        Ok(())
    }
}

#[cfg(feature = "crc64fast-nvme")]
#[cfg(test)]
#[allow(clippy::redundant_clone)]
mod tests_crc64fast_nvme {
    use std::io::Write;

    use crate::config::CodeqConfig;
    use crate::config::Crc64fastNvme;

    #[test]
    fn test_checksum_writer_crc64fast_nvme() -> anyhow::Result<()> {
        let mut b = Vec::new();

        let mut n = 0;
        let mut w = Crc64fastNvme::new_writer(&mut b);
        n += w.write(b"foo")?;
        n += w.write_checksum()?;

        assert_eq!(n, 11);
        assert_eq!(
            vec![
                102, 111, 111, //
                228, 237, 247, 14, 102, 174, 13, 2
            ],
            b
        );

        Ok(())
    }
}
