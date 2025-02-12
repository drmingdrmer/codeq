mod decode;
mod encode;
mod impls;

pub use decode::Decode;
pub use encode::Encode;

/// A trait that is [`Encode`] and [`Decode`].
///
/// A type that is both [`Encode`] and [`Decode`] is automatically implemented as a [`Codec`].
pub trait Codec: Encode + Decode {}

impl<T> Codec for T where T: Encode + Decode {}
