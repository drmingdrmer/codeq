use std::io;

pub struct OffsetWriter<W> {
    inner: W,
    offset: usize,
}

impl<W: io::Write> OffsetWriter<W> {
    pub fn new(inner: W) -> Self {
        Self { inner, offset: 0 }
    }

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
