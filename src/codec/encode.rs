use std::io;
use std::io::Error;
use std::io::Write;

pub trait Encode: Sized {
    fn encode<W: io::Write>(&self, w: W) -> Result<usize, io::Error>;
}

impl<T: Encode> Encode for &T {
    fn encode<W: Write>(&self, w: W) -> Result<usize, Error> {
        (*self).encode(w)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Error;
    use std::io::Write;

    use crate::codec::Encode;

    struct Foo;

    impl Encode for Foo {
        fn encode<W: Write>(&self, _w: W) -> Result<usize, Error> {
            Ok(3)
        }
    }

    #[test]
    fn test_encode_ref() {
        let foo = Foo;
        let n = Encode::encode(&foo, Vec::new()).unwrap();
        assert_eq!(n, 3);

        let n = Encode::encode(&&foo, Vec::new()).unwrap();
        assert_eq!(n, 3);
    }
}
