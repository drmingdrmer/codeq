use std::io;

use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;

use crate::Decode;
use crate::Encode;

impl Encode for &str {
    fn encode<W: io::Write>(&self, mut w: W) -> Result<usize, io::Error> {
        let bytes = self.as_bytes();
        w.write_u32::<BigEndian>(bytes.len() as u32)?;
        w.write_all(bytes)?;
        Ok(bytes.len() + 4)
    }
}

impl Encode for String {
    fn encode<W: io::Write>(&self, w: W) -> Result<usize, io::Error> {
        self.as_str().encode(w)
    }
}

impl Decode for String {
    fn decode<R: io::Read>(mut r: R) -> Result<Self, io::Error> {
        let len = r.read_u32::<BigEndian>()? as usize;
        let mut buf = vec![0; len];
        r.read_exact(&mut buf)?;
        String::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

#[cfg(test)]
mod tests {

    use std::io;

    use crate::Decode;
    use crate::Encode;

    #[test]
    fn test_string_codec() -> Result<(), io::Error> {
        let s = "hello".to_string();
        let mut buf = Vec::new();
        let n = s.encode(&mut buf)?;
        assert_eq!(n, buf.len());
        assert_eq!(buf.len(), 4 + s.len());

        let b = String::decode(&mut buf.as_slice())?;
        assert_eq!(s, b);

        Ok(())
    }

    #[test]
    fn test_str_encode() -> Result<(), io::Error> {
        check_str_encode("hello", b"\x00\x00\x00\x05hello")
    }

    #[test]
    fn test_str_encode_empty() -> Result<(), io::Error> {
        check_str_encode("", b"\x00\x00\x00\x00")
    }

    #[test]
    fn test_str_encode_multibyte() -> Result<(), io::Error> {
        let s = "你好";
        assert_eq!(s.chars().count(), 2, "2 characters");
        assert_eq!(s.len(), 6, "but 6 UTF-8 bytes");

        let mut buf = Vec::new();
        s.encode(&mut buf)?;

        // Length prefix counts bytes (6), not characters (2).
        assert_eq!(&buf[..4], &[0u8, 0, 0, 6]);
        assert_eq!(&buf[4..], s.as_bytes());
        assert_eq!(String::decode(&mut buf.as_slice())?, s);

        Ok(())
    }

    #[test]
    fn test_str_matches_string_encode() -> Result<(), io::Error> {
        for s in ["", "hello", "你好"] {
            let mut from_str = Vec::new();
            s.encode(&mut from_str)?;

            let mut from_string = Vec::new();
            s.to_string().encode(&mut from_string)?;

            assert_eq!(
                from_str, from_string,
                "&str and String must encode identically: {s:?}"
            );
        }
        Ok(())
    }

    /// Encodes `s`, asserting the exact wire bytes, the returned length, and a
    /// clean round-trip back through `String::decode`.
    fn check_str_encode(s: &str, expect: &[u8]) -> Result<(), io::Error> {
        let mut buf = Vec::new();
        let n = s.encode(&mut buf)?;

        assert_eq!(buf, expect, "exact wire bytes for {s:?}");
        assert_eq!(n, buf.len(), "returned count equals bytes written");
        assert_eq!(buf.len(), 4 + s.len(), "4-byte prefix + UTF-8 byte length");

        let decoded = String::decode(&mut buf.as_slice())?;
        assert_eq!(decoded, s, "round-trips through String::decode");

        Ok(())
    }
}
