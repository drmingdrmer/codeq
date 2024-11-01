use std::io;

pub struct OffsetReader<R> {
    inner: R,
    offset: usize,
}

impl<R: io::Read> OffsetReader<R> {
    pub fn new(inner: R) -> Self {
        Self { inner, offset: 0 }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}

impl<R: io::Read> io::Read for OffsetReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(buf)?;
        self.offset += n;
        Ok(n)
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Read;

    use crate::offset_reader::OffsetReader;

    #[test]
    fn test_offset_reader() -> Result<(), io::Error> {
        let data = b"hello";
        let mut reader = OffsetReader::new(data.as_ref());
        let mut buf = [0; 3];
        reader.read_exact(&mut buf)?;
        assert_eq!(reader.offset(), 3);

        let mut buf = [0; 2];
        reader.read_exact(&mut buf)?;
        assert_eq!(reader.offset(), 5);

        Ok(())
    }
}
