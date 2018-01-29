//! A basic ZipReader/Writer crate

#![warn(missing_docs)]

#[cfg(feature = "bzip2")]
extern crate bzip2;
#[cfg(feature = "flate2")]
extern crate flate2;
#[cfg(feature = "inflate")]
extern crate inflate;
extern crate podio;

pub use read::ZipArchive;
pub use write::ZipWriter;
pub use compression::CompressionMethod;

pub mod spec;
mod crc32;
pub mod types;
pub mod read;
mod compression;
pub mod write;
mod cp437;
pub mod result;
