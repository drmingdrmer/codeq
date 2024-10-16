use std::io;

use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;

use crate::Decode;
use crate::Encode;

impl Encode for String {
    fn encode<W: io::Write>(&self, mut w: W) -> Result<usize, io::Error> {
        let bytes = self.as_bytes();
        w.write_u32::<BigEndian>(bytes.len() as u32)?;
        w.write_all(bytes)?;
        Ok(bytes.len() + 4)
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
}
