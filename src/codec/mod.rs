mod decode;
mod encode;
mod impls;

pub use decode::Decode;
pub use encode::Encode;

pub trait Codec: Encode + Decode {}

impl<T> Codec for T where T: Encode + Decode {}
