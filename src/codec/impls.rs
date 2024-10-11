use std::io;
use std::io::Error;
use std::io::Read;

use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;

use crate::codec::Decode;
use crate::codec::Encode;
use crate::FixedSize;

impl Encode for u64 {
    fn encode<W: io::Write>(&self, mut w: W) -> Result<usize, io::Error> {
        w.write_u64::<byteorder::BigEndian>(*self)?;
        Ok(Self::encoded_size())
    }
}

impl Decode for u64 {
    fn decode<R: io::Read>(mut r: R) -> Result<Self, io::Error> {
        let v = r.read_u64::<byteorder::BigEndian>()?;
        Ok(v)
    }
}

impl Encode for u32 {
    fn encode<W: io::Write>(&self, mut w: W) -> Result<usize, io::Error> {
        w.write_u32::<byteorder::BigEndian>(*self)?;
        Ok(Self::encoded_size())
    }
}

impl Decode for u32 {
    fn decode<R: io::Read>(mut r: R) -> Result<Self, io::Error> {
        let v = r.read_u32::<byteorder::BigEndian>()?;
        Ok(v)
    }
}

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

impl<A: Encode, B: Encode> Encode for (A, B) {
    fn encode<W: io::Write>(&self, mut w: W) -> Result<usize, io::Error> {
        let mut n = 0;
        n += self.0.encode(&mut w)?;
        n += self.1.encode(&mut w)?;
        Ok(n)
    }
}

impl<A: Decode, B: Decode> Decode for (A, B) {
    fn decode<R: io::Read>(mut r: R) -> Result<Self, io::Error> {
        let a = A::decode(&mut r)?;
        let b = B::decode(&mut r)?;
        Ok((a, b))
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use std::io;
    use std::mem::size_of;

    use crate::codec::Codec;
    use crate::fixed_size::FixedSize;
    use crate::Decode;
    use crate::Encode;

    #[test]
    fn test_u64_codec() -> anyhow::Result<()> {
        test_int_coded(0x1234567890abcdefu64)
    }

    #[test]
    fn test_u32_codec() -> anyhow::Result<()> {
        test_int_coded(0x12345678u32)
    }

    fn test_int_coded<T: Codec + FixedSize + PartialEq + Debug>(v: T) -> anyhow::Result<()> {
        let size = size_of::<T>();

        assert_eq!(T::encoded_size(), size);

        let mut buf = Vec::new();
        let n = v.encode(&mut buf)?;
        assert_eq!(n, buf.len());

        let b = T::decode(&mut buf.as_slice())?;
        assert_eq!(v, b);

        Ok(())
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
    fn test_tuple_2_codec() -> Result<(), io::Error> {
        let v = (1u64, 2u32);
        let mut buf = Vec::new();
        let n = v.encode(&mut buf)?;
        assert_eq!(n, buf.len());
        assert_eq!(buf.len(), 8 + 4);

        let b = <(u64, u32)>::decode(&mut buf.as_slice())?;
        assert_eq!(v, b);

        Ok(())
    }
}
