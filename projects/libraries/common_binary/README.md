# common_binary

Stable high-performance binary persistence ABI for the workspace.

## Overview

`common_binary` provides a single canonical binary persistence API that is:

- **Performance-first**: Fast load/store, minimal allocations
- **Safe**: Detects corruption and incompatibility early
- **Backend-agnostic**: Implementation hidden behind the ABI

## Features

- Fixed-size header with metadata and checksums
- Schema versioning support
- Corruption detection via fast checksums
- Clean error handling
- Backend encapsulation (currently uses bincode internally)

## Usage

```rust
use common_binary::{BinaryOptions, BinaryEncode, BinaryDecode, write_binary, read_binary};
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct MyData {
    value: u64,
    name: String,
}

// BinaryEncode and BinaryDecode are automatically implemented
// for any type that implements Serialize and Deserialize

// Write to file
let data = MyData { value: 42, name: "test".to_string() };
let opts = BinaryOptions {
    magic: *b"MYDT",
    container_version: 1,
    schema_id: 1,
    verify_checksum: true,
};
write_binary(&data, "data.bin", &opts)?;

// Read from file
let loaded: MyData = read_binary("data.bin", &opts)?;
assert_eq!(data, loaded);
```

## Schema Management

The `schema_id` field should be bumped whenever you make breaking changes to your data structure:

```rust
const MY_DATA_SCHEMA_V1: u64 = 1;
const MY_DATA_SCHEMA_V2: u64 = 2; // Bumped after adding new field

let opts = BinaryOptions {
    magic: *b"MYDT",
    container_version: 1,
    schema_id: MY_DATA_SCHEMA_V2,
    verify_checksum: true,
};
```

## Binary Container Format

The binary file format consists of:

1. **Header (32 bytes)**:
   - Magic (4 bytes) - File type identifier
   - Container version (2 bytes) - Binary format version
   - Flags (2 bytes) - Reserved for future use
   - Schema ID (8 bytes) - Application-defined schema version
   - Payload length (8 bytes) - Length of payload in bytes
   - Checksum (8 bytes) - FNV-1a hash of payload

2. **Payload**: Encoded data (length specified in header)

## Error Handling

All operations return `Result<T, BinaryError>` where `BinaryError` can be:

- `Io`: File I/O errors
- `Corrupt`: Data corruption detected (e.g., checksum mismatch)
- `Incompatible`: Version or schema mismatch
- `Encode`: Encoding error
- `Decode`: Decoding error

## Design Principles

1. **No partial loads**: Either the entire file is valid and loaded, or an error is returned
2. **Early validation**: Invalid files are rejected immediately during header parsing
3. **Backend encapsulation**: The binary serialization format is internal and not exposed in the public API
4. **Minimal dependencies**: Only depends on serde and thiserror
5. **Serde integration**: Works seamlessly with any type implementing Serialize/Deserialize

## Non-goals

- ❌ Human-readable formats (use `common_json` instead)
- ❌ Network protocol standardization
- ❌ Infinite backward compatibility
- ❌ Cross-language support
