//! Configuration for checksum calculation and verification.

use core::hash::Hasher;
use std::fmt::Debug;
use std::hash::Hash;
use std::io;

use crate::ChecksumReader;
use crate::ChecksumWriter;
use crate::Segment;
use crate::WithChecksum;

/// Static Configuration for checksum calculation and verification.
///
/// This trait defines how checksums are calculated and verified for data integrity.
/// It allows applications to:
/// - Choose their preferred checksum algorithm (e.g., CRC32, CRC64)
/// - Create checksum-enabled readers and writers
/// - Wrap data with checksums
///
/// # Examples
///
/// Using the default CRC32 implementation:
#[cfg_attr(not(feature = "crc32fast"), doc = "```ignore")]
#[cfg_attr(feature = "crc32fast", doc = "```rust")]
/// use codeq::config::Crc32fast;
/// # use codeq::config::CodeqConfig;
/// # use std::io::Write;
///
/// let mut writer = Crc32fast::new_writer(Vec::new());
/// writer.write_all(b"hello").unwrap();
/// ```
/// 
/// Custom implementation:
/// ```
/// use codeq::config::CodeqConfig;
///
/// #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// struct CustomConfig;
/// impl CodeqConfig for CustomConfig {
///     type Hasher = std::collections::hash_map::DefaultHasher;
/// }
/// ```
/// 
/// Note: Data encoded with one configuration cannot be decoded with a different configuration.
/// For example, data encoded with CRC32 cannot be decoded with CRC64, and vice versa.
///
/// Standard bounds (`Debug`, `Clone`, `Default`, etc.) required for use as a generic parameter
/// in types like `WithChecksum<C,T>` and `Segment<C>` throughout the codebase.
pub trait CodeqConfig
where Self: Debug + Clone + Copy + Default + PartialEq + Eq + PartialOrd + Ord + Hash + Sized
{
    /// The hasher type used for checksum calculation.
    type Hasher: Hasher + Default;

    /// Calculates a checksum for the given buffer.
    fn hash(buf: &[u8]) -> u64 {
        let mut hasher = Self::Hasher::default();
        hasher.write(buf);
        hasher.finish()
    }

    /// Creates a new checksum writer wrapping the given writer.
    fn new_writer<W: io::Write>(inner: W) -> ChecksumWriter<Self, W> {
        ChecksumWriter::new(inner)
    }

    /// Creates a new checksum reader wrapping the given reader.
    fn new_reader<R: io::Read>(inner: R) -> ChecksumReader<Self, R> {
        ChecksumReader::new(inner)
    }

    /// Wraps data with checksum protection.
    fn wrap<T>(data: T) -> WithChecksum<Self, T> {
        WithChecksum::<Self, _>::new(data)
    }

    /// Creates a new segment with the given offset and size.
    fn segment(offset: u64, size: u64) -> Segment<Self> {
        Segment::<Self>::new(offset, size)
    }
}

#[cfg(feature = "crc32fast")]
pub mod crc32fast_impl {
    use super::CodeqConfig;

    /// CRC32 checksum implementation.
    ///
    /// Derives standard traits to allow this type to be used as a generic type parameter,
    /// when the containing type requires these bounds.
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Crc32fast;

    impl CodeqConfig for Crc32fast {
        type Hasher = crc32fast::Hasher;
    }
}

#[cfg(feature = "crc32fast")]
pub use crc32fast_impl::Crc32fast;

#[cfg(feature = "crc64fast-nvme")]
mod crc64fast_nvme_impl {
    use crate::config::CodeqConfig;

    #[derive(Default, Clone)]
    pub struct Crc64fastNvmeHasher(crc64fast_nvme::Digest);

    impl core::hash::Hasher for Crc64fastNvmeHasher {
        fn finish(&self) -> u64 {
            self.0.sum64()
        }

        fn write(&mut self, bytes: &[u8]) {
            self.0.write(bytes);
        }
    }

    /// CRC64-NVME checksum implementation.
    ///
    /// Derives standard traits to allow this type to be used as a generic type parameter,
    /// when the containing type requires these bounds.
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Crc64fastNvme;

    impl CodeqConfig for Crc64fastNvme {
        type Hasher = Crc64fastNvmeHasher;
    }
}

#[cfg(feature = "crc64fast-nvme")]
pub use crc64fast_nvme_impl::Crc64fastNvme;
