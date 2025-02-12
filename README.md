# codeq

A binary codec library optimized for storage and networking, with built-in checksum support and file operation utilities.

## Features

- 🔒 **Data Integrity**: Built-in CRC32 checksum support for detecting data corruption
- 📍 **Position Tracking**: Type-safe offset and size handling for file operations
- ⚡ **Performance**: Optimized for binary format with fixed-size type support
- 🛠️ **Simple API**: Direct encoding/decoding without intermediate formats
- 📦 **Zero Dependencies**: Minimal dependency footprint

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
codeq = "0.1"
```

### Basic Example

```rust
use codeq::{Codec, Decode, Encode, WithChecksum};
use std::io;

#[derive(Debug, Clone, PartialEq)]
struct Record {
    id: u32,
    data: Vec<u8>,
}

impl Encode for Record {
    fn encode<W: io::Write>(&self, mut writer: W) -> io::Result<usize> {
        let mut n = 0;
        n += self.id.encode(&mut writer)?;
        n += self.data.encode(&mut writer)?;
        Ok(n)
    }
}
impl Decode for Record {
    fn decode<R: io::Read>(mut reader: R) -> io::Result<Self> {
        Ok(Self {
            id: u32::decode(&mut reader)?,
            data: Vec::decode(&mut reader)?,
        })
    }
}

// Add checksum protection
let record = Record { id: 1, data: vec![1, 2, 3] };
let protected = WithChecksum::new(&record);

let mut buf = Vec::new();
protected.encode(&mut buf).unwrap();
assert_eq!(buf, vec![ //
    0, 0, 0, 1, // id
    0, 0, 0, 3, 1, 2, 3, // data
    0, 0, 0, 0, 31, 101, 71, 147 // checksum
]);

let decoded = Record::decode(&mut buf.as_slice()).unwrap();
assert_eq!(record, decoded);
```

## When to Use codeq vs serde

Choose **codeq** when you need:
- Efficient binary serialization with checksums
- Direct control over the binary format
- File/buffer position tracking
- Simple implementation without format abstraction

Use **serde** when you need:
- Multiple format support (JSON, YAML, etc.)
- Derive macros for automatic implementation
- Complex data structure serialization
- Format-agnostic code