use std::io;

use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;

use crate::Decode;
use crate::Encode;
use crate::FixedSize;

impl FixedSize for bool {
    fn encoded_size() -> usize {
        1
    }
}

impl Encode for bool {
    fn encode<W: io::Write>(&self, mut w: W) -> Result<usize, io::Error> {
        w.write_u8(if *self { 1 } else { 0 })?;
        Ok(1)
    }
}

impl Decode for bool {
    fn decode<R: io::Read>(mut r: R) -> Result<Self, io::Error> {
        let b = r.read_u8()?;
        if b > 1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid bool value: {}", b),
            ));
        }
        Ok(b != 0)
    }
}

#[cfg(test)]
mod tests {

    use std::io;

    use crate::Decode;
    use crate::Encode;
    use crate::FixedSize;

    #[test]
    fn test_bool_codec() -> Result<(), io::Error> {
        let b = true;

        assert_eq!(1, bool::encoded_size());

        let mut buf = Vec::new();
        let n = b.encode(&mut buf)?;
        assert_eq!(n, buf.len());
        assert_eq!(buf.len(), 1);

        let b2 = bool::decode(&mut buf.as_slice())?;
        assert_eq!(b, b2);

        Ok(())
    }
}
