extern crate core;

mod checksum_reader;
mod checksum_writer;
mod codec;
mod fixed_size;
mod offset_reader;
mod offset_writer;
mod segment;
mod span;
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
pub use offset_writer::OffsetWriter;
pub use segment::Segment;
pub use span::Offset;
pub use span::Size;
pub use span::Span;
// Backward compatibility
pub use span::Span as OffsetSize;
pub use with_checksum::WithChecksum;
