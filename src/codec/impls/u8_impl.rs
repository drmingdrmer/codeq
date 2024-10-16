use std::io;
use std::mem::size_of;

use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;

use crate::Decode;
use crate::Encode;
use crate::FixedSize;

impl FixedSize for u8 {
    fn encoded_size() -> usize {
        size_of::<Self>()
    }
}

impl Encode for u8 {
    fn encode<W: io::Write>(&self, mut w: W) -> Result<usize, io::Error> {
        w.write_u8(*self)?;
        Ok(Self::encoded_size())
    }
}

impl Decode for u8 {
    fn decode<R: io::Read>(mut r: R) -> Result<Self, io::Error> {
        let v = r.read_u8()?;
        Ok(v)
    }
}

#[cfg(test)]
mod tests {

    use crate::testing::test_int_coded;

    #[test]
    fn test_u8_codec() -> anyhow::Result<()> {
        test_int_coded(0x12u8)
    }
}
