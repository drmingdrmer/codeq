//! A fully customized encode/decode library.
//!
//! **Differences from serde**: This crate focuses on a single, application defined binary format,
//! built-in checksum support, simple encoding/decoding without intermediate formats,
//! and application defined compatibility.
//!
//! Choose **this crate** when you need customized and simple binary serialization with checksums,
//! or when data format evolves over time.
//! Use **serde** when you need: multiple format support (JSON, YAML, etc.),
//! or derive macros for automatic implementation.
//!
//! # Core Traits
//!
//! - [`Codec`], [`Encode`], [`Decode`]: Main trait for types that can be encoded/decoded
//! - [`FixedSize`]: For types with known encoded size
//! - [`Span`]: For types representing a region in a file/buffer
//!
//! # Utilities
//!
//! - [`ChecksumReader`]/[`ChecksumWriter`]: I/O wrappers that calculate checksums
//! - [`WithChecksum<T>`]: Wraps data with checksum for integrity
//! - [`Offset`]: Type-safe byte position in a file/buffer
//! - [`Size`]: Type-safe byte length
//! - [`OffsetReader`]/[`OffsetWriter`]: I/O wrappers that track current position
//! - [`Segment<T>`]: Represents a typed region with offset and size
//!
//! # Examples
//!
//! Basic encoding and decoding:
#![cfg_attr(not(feature = "crc32fast"), doc = "```ignore")]
#![cfg_attr(feature = "crc32fast", doc = "```rust")]
//! use codeq::{Codec, Decode, Encode, WithChecksum};
//! # use std::io;
//!
//! use codeq::config::Crc32fast;
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
//! let protected = WithChecksum::<Crc32fast,_>::new(&record);
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

pub mod config;
pub mod error_context_ext;
pub(crate) mod sealed;
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
