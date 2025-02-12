//! A binary codec library optimized for storage and networking.
//!
//! This crate provides traits and implementations for encoding and decoding data
//! in a binary format, with special focus on:
//! - Data integrity through CRC32 checksums
//! - Efficient handling of file/buffer offsets and sizes
//!
//! # Core Traits
//!
//! - [`Codec`], [`Encode`], [`Decode`]: Main trait for types that can be encoded/decoded
//! - [`FixedSize`]: For types with known encoded size
//! - [`Span`]: For types representing a region in a file/buffer
//!
//! # Key Types
//!
//! - [`WithChecksum<T>`]: Wraps data with CRC32 checksum for integrity
//! - [`Offset`]: Type-safe byte position in a file/buffer
//! - [`Size`]: Type-safe byte length
//! - [`Segment<T>`]: Represents a typed region with offset and size
//!
//! # Utilities
//!
//! - [`ChecksumReader`]/[`ChecksumWriter`]: I/O wrappers that calculate checksums
//! - [`OffsetReader`]/[`OffsetWriter`]: I/O wrappers that track current position
//!
//! # Examples
//!
//! Basic encoding and decoding:
//! ```
//! use codeq::{Codec, Decode, Encode, WithChecksum};
//! # use std::io;
//!
//! #[derive(Debug, Clone, PartialEq)]
//! struct Record {
//!     id: u32,
//!     data: Vec<u8>,
//! }
//!
//! impl Encode for Record {
//!     fn encode<W: io::Write>(&self, mut writer: W) -> io::Result<usize> {
//!         let mut n = 0;
//!         n += self.id.encode(&mut writer)?;
//!         n += self.data.encode(&mut writer)?;
//!         Ok(n)
//!     }
//! }
//! impl Decode for Record {
//!     fn decode<R: io::Read>(mut reader: R) -> io::Result<Self> {
//!         Ok(Self {
//!             id: u32::decode(&mut reader)?,
//!             data: Vec::decode(&mut reader)?,
//!         })
//!     }
//! }
//!
//! // Add checksum protection
//! let record = Record { id: 1, data: vec![1, 2, 3] };
//! let protected = WithChecksum::new(&record);
//!
//! let mut buf = Vec::new();
//! protected.encode(&mut buf).unwrap();
//! assert_eq!(buf, vec![ //
//!     0, 0, 0, 1, // id
//!     0, 0, 0, 3, 1, 2, 3, // data
//!     0, 0, 0, 0, 31, 101, 71, 147 // checksum
//! ]);
//!
//! let decoded = Record::decode(&mut buf.as_slice()).unwrap();
//! assert_eq!(record, decoded);
//! ```
//!
//! # Differences from serde
//!
//! While serde is a general-purpose serialization framework, this crate is specialized for:
//!
//! 1. **Single Format**: Only supports a single, efficient binary format
//! 1. **Data Integrity**: Built-in checksum support for detecting corruption
//! 1. **File Operations**: First-class support for file offsets and regions
//! 1. **Simple Implementation**: Direct encoding/decoding without intermediate formats
//!
//! Choose this crate when you need:
//! - Efficient binary serialization with checksums
//! - Direct control over the binary format
//! - File/buffer position tracking
//! - Simple implementation without format abstraction
//!
//! Use serde when you need:
//! - Multiple format support (JSON, YAML, etc.)
//! - Derive macros for automatic implementation
//! - Complex data structure serialization
//! - Format-agnostic code
//!
//! [`Codec`]: crate::Codec
//! [`Encode`]: crate::Encode
//! [`Decode`]: crate::Decode
//! [`FixedSize`]: crate::FixedSize
//! [`Span`]: crate::Span
//! [`Offset`]: crate::Offset
//! [`Size`]: crate::Size
//! [`Segment<T>`]: crate::Segment
//! [`WithChecksum<T>`]: crate::WithChecksum
//! [`ChecksumReader`]: crate::ChecksumReader
//! [`ChecksumWriter`]: crate::ChecksumWriter
//! [`OffsetReader`]: crate::OffsetReader
//! [`OffsetWriter`]: crate::OffsetWriter

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
