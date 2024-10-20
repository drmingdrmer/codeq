use std::io::Error;
use std::io::Read;
use std::io::Write;

use crate::Decode;
use crate::Encode;
use crate::FixedSize;

impl<T: FixedSize> FixedSize for Option<T> {
    fn encoded_size() -> usize {
        1 + T::encoded_size()
    }
}

impl<T: Encode> Encode for Option<T> {
    fn encode<W: Write>(&self, mut w: W) -> Result<usize, Error> {
        match self {
            Some(v) => {
                let n = 1u8.encode(&mut w)? + v.encode(&mut w)?;
                Ok(n)
            }
            None => 0u8.encode(&mut w),
        }
    }
}

impl<T: Decode> Decode for Option<T> {
    fn decode<R: Read>(mut r: R) -> Result<Self, Error> {
        let tag = u8::decode(&mut r)?;
        match tag {
            0 => Ok(None),
            1 => {
                let v = T::decode(&mut r)?;
                Ok(Some(v))
            }
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid tag: {}", tag),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use crate::Decode;
    use crate::Encode;

    #[test]
    fn test_option_codec() -> Result<(), io::Error> {
        //
        {
            let a = Some("foo".to_string());

            let mut b = Vec::new();
            let n = a.encode(&mut b)?;
            assert_eq!(n, 8);
            assert_eq!(b, vec![1, 0, 0, 0, 3, 102, 111, 111]);

            let c = Option::<String>::decode(&mut b.as_slice())?;
            assert_eq!(c, a);
        }

        {
            let a = None::<String>;

            let mut b = Vec::new();
            let n = a.encode(&mut b)?;
            assert_eq!(n, 1);
            assert_eq!(b, vec![0]);

            let c = Option::<String>::decode(&mut b.as_slice())?;
            assert_eq!(c, a);
        }

        Ok(())
    }
}
