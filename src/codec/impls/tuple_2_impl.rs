use std::io;

use crate::Decode;
use crate::Encode;

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
    use std::io;

    use crate::Decode;
    use crate::Encode;

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
