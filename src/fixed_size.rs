/// Trait for types that have a fixed, known size when encoded.
///
/// This trait is implemented by types whose encoded representation always
/// has the same size, regardless of their value. For example:
/// - Primitive integers (u8, i32, u64, etc.)
/// - Fixed-size structs containing only fixed-size fields
/// - Arrays or tuples of fixed-size types
///
/// This information can be used to:
/// - Pre-allocate buffers of the correct size
/// - Perform bounds checking before encoding/decoding
/// - Calculate storage requirements statically
pub trait FixedSize {
    /// Returns the size in bytes that this type will occupy when encoded.
    ///
    /// This size must be constant for all instances of the type.
    fn encoded_size() -> usize;
}
