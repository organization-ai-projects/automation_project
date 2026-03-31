//! # ap_file_format
//!
//! Custom file format library for the workspace.
//!
//! Provides a single container format (`.apf`) that supports multiple content
//! types including plain text, Markdown, JSON, RON, raw binary, and images.
//!
//! ## Features
//!
//! - Fixed-size 32-byte header with magic bytes, version, and checksums
//! - Content type tags for text, Markdown, JSON, RON, binary, and image data
//! - Image support with pixel format metadata (Gray8, RGB8, RGBA8)
//! - Schema versioning for forward compatibility
//! - Corruption detection via FNV-1a checksums
//! - Atomic writes (temp file + rename)
//!
//! ## Example
//!
//! ```rust,no_run
//! use ap_file_format::{ApFileOptions, write_text, read_text};
//!
//! let opts = ApFileOptions::default();
//! write_text("hello.apf", "Hello, world!", &opts).unwrap();
//! let text = read_text("hello.apf", &opts).unwrap();
//! assert_eq!(text, "Hello, world!");
//! ```

pub mod content_type;
pub mod error;
mod header;
pub mod image;
pub mod io;

pub use content_type::ContentType;
pub use error::{ApFileError, ApFileResult};
pub use image::{ImageData, PixelFormat};
pub use io::{
    ApFileOptions, read_binary, read_image, read_json, read_markdown, read_raw, read_ron,
    read_text, write_binary, write_image, write_json, write_markdown, write_ron, write_text,
};

#[cfg(test)]
mod tests;
