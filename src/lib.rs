extern crate core;

mod checksum_reader;
mod checksum_writer;
mod codec;
mod fixed_size;
mod offset_reader;
mod offset_size;
mod offset_writer;
mod segment;
mod with_checksum;

pub mod error_context_ext;
pub mod testing;

pub use checksum_reader::ChecksumReader;
pub use checksum_writer::ChecksumWriter;
pub use codec::Codec;
pub use codec::Decode;
pub use codec::Encode;
pub use fixed_size::FixedSize;
pub use offset_reader::OffsetReader;
pub use offset_size::Offset;
pub use offset_size::Size;
pub use offset_size::Span;
// Backward compatibility
pub use offset_size::Span as OffsetSize;
pub use offset_writer::OffsetWriter;
pub use segment::Segment;
pub use with_checksum::WithChecksum;
