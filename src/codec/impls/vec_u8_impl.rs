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
        let mut buf = vec![0; len];
        r.read_exact(&mut buf)?;
        Ok(buf)
    }
}

#[cfg(test)]
mod tests {

    use std::io;

    use crate::Decode;
    use crate::Encode;

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
}
