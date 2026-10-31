use std::io;
use std::io::Error;
use std::io::Read;

use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;

use crate::Decode;
use crate::Encode;

impl Encode for Vec<u8> {
    fn encode<W: io::Write>(&self, mut w: W) -> Result<usize, io::Error> {
        w.write_u32::<BigEndian>(self.len() as u32)?;
        w.write_all(self)?;
        Ok(self.len() + 4)
    }
}

impl Decode for Vec<u8> {
    fn decode<R: Read>(mut r: R) -> Result<Self, Error> {
        let len = r.read_u32::<BigEndian>()? as usize;
        let mut buf = Vec::new();
        let mut reader = r.take(len as u64);
        reader.read_to_end(&mut buf)?;
        if buf.len() != len {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }
        Ok(buf)
    }
}

#[cfg(test)]
mod tests {

    use std::io;
    use std::io::Read;

    use crate::Decode;
    use crate::Encode;

    const DECLARED_LEN: u32 = 1024 * 1024;

    struct ShortReader<'a>(&'a [u8]);

    impl Read for ShortReader<'_> {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let bounded = buf.len() < DECLARED_LEN as usize;
            assert!(bounded);
            let len = buf.len().min(1);
            self.0.read(&mut buf[..len])
        }
    }

    #[test]
    fn test_vec_u8_codec() -> Result<(), io::Error> {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut buf = Vec::new();
        let n = v.encode(&mut buf)?;
        assert_eq!(n, buf.len());
        assert_eq!(buf.len(), 4 + v.len());

        let b = Vec::<u8>::decode(&mut buf.as_slice())?;
        assert_eq!(v, b);

        Ok(())
    }

    #[test]
    fn test_decode_does_not_request_declared_length() {
        let prefix = DECLARED_LEN.to_be_bytes();
        let result = Vec::<u8>::decode(ShortReader(&prefix));
        let error = result.unwrap_err();
        let kind = error.kind();
        assert_eq!(kind, io::ErrorKind::UnexpectedEof);

        let result = String::decode(ShortReader(&prefix));
        let error = result.unwrap_err();
        let kind = error.kind();
        assert_eq!(kind, io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn test_decode_truncated() {
        for data in [
            b"".as_slice(),
            b"\x00\x00\x00".as_slice(),
            b"\x00\x00\x00\x05hi".as_slice(),
            b"\xff\xff\xff\xff".as_slice(),
            b"\xff\xff\xff\xffhi".as_slice(),
        ] {
            let result = Vec::<u8>::decode(data);
            let error = result.unwrap_err();
            let kind = error.kind();
            assert_eq!(kind, io::ErrorKind::UnexpectedEof);

            let result = String::decode(data);
            let error = result.unwrap_err();
            let kind = error.kind();
            assert_eq!(kind, io::ErrorKind::UnexpectedEof);
        }
    }

    #[test]
    fn test_vec_u8_decode_short_reads_leaves_trailing_bytes() -> Result<(), io::Error> {
        let data = b"\x00\x00\x00\x00\x00\x00\x00\x03\x00\xff\x01tail";
        let mut reader = ShortReader(data);
        let empty = Vec::<u8>::decode(&mut reader)?;
        assert_eq!(empty, b"");

        let actual = Vec::<u8>::decode(&mut reader)?;
        assert_eq!(actual, b"\x00\xff\x01");
        assert_eq!(reader.0, b"tail");
        Ok(())
    }
}
