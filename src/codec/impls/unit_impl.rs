use std::io;

use crate::Decode;
use crate::Encode;
use crate::FixedSize;

impl FixedSize for () {
    fn encoded_size() -> usize {
        0
    }
}

impl Encode for () {
    fn encode<W: io::Write>(&self, _w: W) -> Result<usize, io::Error> {
        Ok(0)
    }
}

impl Decode for () {
    fn decode<R: io::Read>(_r: R) -> Result<Self, io::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Decode;
    use crate::Encode;
    use crate::FixedSize;

    #[test]
    fn test_unit_fixed_size() {
        assert_eq!(<()>::encoded_size(), 0);
    }

    #[allow(clippy::unit_cmp)]
    #[allow(clippy::let_unit_value)]
    #[test]
    fn test_unit_codec() -> Result<(), io::Error> {
        let u = ();
        let mut buf = Vec::new();
        let n = u.encode(&mut buf)?;
        assert_eq!(n, 0);
        assert_eq!(buf.len(), 0);

        let b = <()>::decode(&mut buf.as_slice())?;
        assert_eq!(u, b);

        Ok(())
    }
}
