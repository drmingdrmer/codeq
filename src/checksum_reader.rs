use std::fmt;
use std::hash::Hasher;
use std::io;

use byteorder::BigEndian;
use byteorder::ReadBytesExt;

use crate::config::Config;

/// A reader wrapper that calculates CRC32 checksum while reading data.
///
/// This reader wraps any type implementing `io::Read` and transparently calculates
/// a CRC32 checksum of all data read through it. The checksum can be either:
/// - Retrieved using [`finalize_checksum()`](Self::finalize_checksum)
/// - Verified against an expected value using [`verify_checksum()`](Self::verify_checksum)
pub struct ChecksumReader<C, R>
where C: Config
{
    hasher: C::Hasher,
    inner: R,
}

impl<C, R> ChecksumReader<C, R>
where
    C: Config,
    R: io::Read,
{
    /// Creates a new `ChecksumReader` wrapping the provided reader.
    pub fn new(inner: R) -> Self {
        Self {
            hasher: C::Hasher::default(),
            inner,
        }
    }

    /// Consumes the reader and returns the calculated CRC32 checksum.
    ///
    /// The returned value is the CRC32 checksum of all data read through this reader.
    #[allow(dead_code)]
    pub fn finalize_checksum(self) -> u64 {
        self.hasher.finish()
    }

    /// Verifies the calculated checksum against an expected value stored in the stream.
    ///
    /// Reads another 8-byte value from the underlying reader and compares
    /// its least significant 32 bits with the calculated checksum. The `context` closure
    /// is called to provide additional context in case of checksum mismatch.
    ///
    /// # Errors
    /// Returns [`io::Error`] with [`io::ErrorKind::InvalidData`] kind if checksums don't match.
    pub fn verify_checksum<D: fmt::Display>(self, context: impl Fn() -> D) -> io::Result<()> {
        let mut r = self.inner;
        let actual = self.hasher.finish();

        let got = r.read_u64::<BigEndian>()?;
        if actual != got {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "crc32 checksum mismatch: expected {:x}, got {:x}, while {}",
                    actual,
                    got,
                    context()
                ),
            ))
        } else {
            Ok(())
        }
    }
}

impl<C, R> io::Read for ChecksumReader<C, R>
where
    C: Config,
    R: io::Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let read = self.inner.read(buf)?;
        if read > 0 {
            self.hasher.write(&buf[..read]);
        }
        Ok(read)
    }
}

#[cfg(feature = "crc32fast")]
#[cfg(test)]
#[allow(clippy::redundant_clone)]
mod tests_crc32fast {
    use std::io::Read;
    use std::io::Write;

    use crate::config::Config;
    use crate::config::Crc32fast;

    #[test]
    fn test_checksum_reader() -> anyhow::Result<()> {
        let mut b = Vec::new();

        // write something
        {
            let mut n = 0;

            let mut w = Crc32fast::new_writer(&mut b);

            n += w.write(b"foo")?;
            n += w.write(b"bar")?;
            n += w.write_checksum()?;
            assert_eq!(n, 14);
        }

        // Finalize the checksum and verify it
        {
            let mut r = Crc32fast::new_reader(&b[..]);
            let mut read_buf = [0u8; 6];
            r.read_exact(&mut read_buf)?;
            let crc = r.finalize_checksum();
            assert_eq!(crc32fast::hash(b"foobar") as u64, crc);
        }

        // Verify the checksum
        {
            let mut r = Crc32fast::new_reader(&b[..]);
            let mut read_buf = [0u8; 6];
            r.read_exact(&mut read_buf)?;
            r.verify_checksum(|| "")?; // No error
        }

        // Verify against wrong checksum
        let last = b.len() - 1;
        b[last] = b[last].wrapping_add(1);

        {
            let mut r = Crc32fast::new_reader(&b[..]);
            let mut read_buf = [0; 6];
            r.read_exact(&mut read_buf)?;
            let res = r.verify_checksum(|| ""); // checksum error
            assert!(res.is_err());
        }

        Ok(())
    }
}

#[cfg(feature = "crc64fast-nvme")]
#[cfg(test)]
#[allow(clippy::redundant_clone)]
mod tests_crc64fast_nvme {
    use std::io::Read;
    use std::io::Write;

    use crate::config::Config;
    use crate::config::Crc64fastNvme;

    #[test]
    fn test_checksum_reader() -> anyhow::Result<()> {
        let mut b = Vec::new();

        // write something
        {
            let mut n = 0;

            let mut w = Crc64fastNvme::new_writer(&mut b);

            n += w.write(b"foo")?;
            n += w.write(b"bar")?;
            n += w.write_checksum()?;
            assert_eq!(n, 14);
        }

        // Finalize the checksum and verify it
        {
            let mut r = Crc64fastNvme::new_reader(&b[..]);
            let mut read_buf = [0u8; 6];
            r.read_exact(&mut read_buf)?;
            let crc = r.finalize_checksum();
            assert_eq!(Crc64fastNvme::hash(b"foobar") as u64, crc);
        }

        // Verify the checksum
        {
            let mut r = Crc64fastNvme::new_reader(&b[..]);
            let mut read_buf = [0u8; 6];
            r.read_exact(&mut read_buf)?;
            r.verify_checksum(|| "")?; // No error
        }

        // Verify against wrong checksum
        let last = b.len() - 1;
        b[last] = b[last].wrapping_add(1);

        {
            let mut r = Crc64fastNvme::new_reader(&b[..]);
            let mut read_buf = [0; 6];
            r.read_exact(&mut read_buf)?;
            let res = r.verify_checksum(|| ""); // checksum error
            assert!(res.is_err());
        }

        Ok(())
    }
}
