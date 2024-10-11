use std::io;

pub trait Decode: Sized {
    fn decode<R: io::Read>(r: R) -> Result<Self, io::Error>;
}
