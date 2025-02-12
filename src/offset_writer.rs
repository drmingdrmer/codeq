use std::io;

/// A writer that tracks the number of bytes written.
///
/// This writer wraps any type implementing [`io::Write`] and transparently tracks
/// the number of bytes written through it. The current offset can be retrieved using
/// the [`offset()`](Self::offset) method.
///
/// Example:
/// ```rust
/// # use std::io::Write;
/// use codeq::OffsetWriter;
///
/// let mut writer = OffsetWriter::new(Vec::new());
/// writer.write_all(b"hello").unwrap();
/// assert_eq!(writer.offset(), 5);
/// ```
pub struct OffsetWriter<W> {
    inner: W,
    offset: usize,
}

impl<W: io::Write> OffsetWriter<W> {
    /// Creates a new `OffsetWriter` wrapping the provided writer.
    ///
    /// # Arguments
    /// * `inner` - The writer to wrap
    ///
    /// # Returns
    pub fn new(inner: W) -> Self {
        Self { inner, offset: 0 }
    }

    /// Returns the current offset of the writer.
    ///
    /// # Returns
    /// The current offset in bytes
    pub fn offset(&self) -> usize {
        self.offset
    }
}

impl<W: io::Write> io::Write for OffsetWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = self.inner.write(buf)?;
        self.offset += n;
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Write;

    use crate::offset_writer::OffsetWriter;

    #[test]
    fn test_offset_writer() -> Result<(), io::Error> {
        let mut buf = Vec::new();
        let mut writer = OffsetWriter::new(&mut buf);
        writer.write_all(b"hello")?;
        assert_eq!(writer.offset(), 5);

        writer.write_all(b"world")?;
        assert_eq!(writer.offset(), 10);

        Ok(())
    }
}
