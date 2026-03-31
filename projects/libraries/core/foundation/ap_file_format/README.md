# ap_file_format

Custom container file format for the workspace.

## Overview

`ap_file_format` provides a single container format (`.apf`) that supports
multiple content types in one crate:

- **Plain text** (UTF-8)
- **Markdown**
- **JSON** (via `common_json`)
- **RON** (via `common_ron`)
- **Raw binary**
- **Images** (Gray8, RGB8, RGBA8 pixel data)

## Features

- Fixed-size 32-byte header with magic bytes (`APFF`), version, and checksums
- Content type tags that identify the payload kind
- Image support with pixel format metadata
- Schema versioning for forward compatibility
- Corruption detection via FNV-1a checksums
- Atomic writes (temp file + rename)

## Usage

```rust
use ap_file_format::{ApFileOptions, write_text, read_text};

let opts = ApFileOptions::default();
write_text("hello.apf", "Hello, world!", &opts)?;
let text = read_text("hello.apf", &opts)?;
assert_eq!(text, "Hello, world!");
```

### JSON

```rust
use ap_file_format::{ApFileOptions, write_json, read_json};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Config { name: String, level: u32 }

let opts = ApFileOptions::default();
let cfg = Config { name: "demo".into(), level: 5 };
write_json("config.apf", &cfg, &opts)?;
let loaded: Config = read_json("config.apf", &opts)?;
assert_eq!(loaded, cfg);
```

### Images

```rust
use ap_file_format::{ApFileOptions, ImageData, PixelFormat, write_image, read_image};

let pixels = vec![255u8; 4 * 4 * 4]; // 4×4 RGBA
let image = ImageData::new(4, 4, PixelFormat::Rgba8, pixels)?;
let opts = ApFileOptions::default();
write_image("tile.apf", &image, &opts)?;
let loaded = read_image("tile.apf", &opts)?;
assert_eq!(loaded, image);
```

## AP Container Format

The binary file format consists of:

1. **Header (32 bytes)**:
   - Magic (4 bytes) — `APFF`
   - Container version (2 bytes)
   - Content type (2 bytes) — identifies payload kind
   - Schema ID (8 bytes) — application-defined schema version
   - Payload length (8 bytes)
   - Checksum (8 bytes) — FNV-1a hash of payload

2. **Payload** — content bytes whose interpretation depends on the content type

### Content types

| Tag | Name       | Payload format                    |
|-----|------------|-----------------------------------|
| 0   | Binary     | Raw bytes                         |
| 1   | PlainText  | UTF-8 text                        |
| 2   | Json       | UTF-8 JSON string                 |
| 3   | Ron        | UTF-8 RON string                  |
| 4   | Image      | 12-byte image sub-header + pixels |
| 5   | Markdown   | UTF-8 Markdown text               |

### Image sub-header (12 bytes)

- Width (4 bytes, little-endian u32)
- Height (4 bytes, little-endian u32)
- Pixel format (1 byte): `0` = Gray8, `1` = RGB8, `2` = RGBA8
- Reserved (3 bytes)

## Error Handling

All operations return `Result<T, ApFileError>` where `ApFileError` can be:

- `Io` — file I/O errors
- `Corrupt` — data corruption detected (checksum mismatch, truncated file)
- `Incompatible` — magic or version mismatch
- `Encode` — serialization error
- `Decode` — deserialization error
- `InvalidContentType` — wrong content type for the requested operation
- `InvalidPath` — target path issue during safe write

## Design Principles

1. **No partial loads** — the entire file is valid or an error is returned
2. **Early validation** — invalid files are rejected during header parsing
3. **Atomic writes** — temp file + rename prevents corruption on crash
4. **Serde integration** — JSON and RON support for any `Serialize`/`Deserialize` type
5. **Minimal dependencies** — only serde, thiserror, common_json, and common_ron
