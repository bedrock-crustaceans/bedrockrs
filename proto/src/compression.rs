use std::io;
use std::io::{Cursor, Write};
use std::sync::Arc;

use crate::error::CompressionError;

#[derive(Clone)]
pub enum Compression {
    Zlib {
        threshold: u16,
        /// Needs to be a number between 0 and 9.
        /// Indicates how compressed the data becomes.
        ///
        /// - 0 = None
        /// - 1 = Fastest
        /// - 6 = Default
        /// - 9 = Best
        compression_level: u8,
    },
    Snappy {
        threshold: u16,
    },
    None,
}

impl Compression {
    /// Used for identifying the compression method used by a given packet
    #[inline]
    pub const fn id_u8(&self) -> u8 {
        match self {
            Compression::Zlib { .. } => 0x00,
            Compression::Snappy { .. } => 0x01,
            Compression::None => 0xFF,
        }
    }

    /// Used in the NetworkSettingsPacket to identify which
    /// CompressionMethod should be used
    #[inline]
    pub const fn id_u16(&self) -> u16 {
        match self {
            Compression::Zlib { .. } => 0x0000,
            Compression::Snappy { .. } => 0x0001,
            Compression::None => 0xFFFF,
        }
    }

    /// Specifies if functions like [`Self::compress`] and [`Self::decompress`] need to be used.
    /// This is needed for optimizing compression.
    #[inline]
    pub const fn compression_needed(&self) -> bool {
        match self {
            Compression::Zlib { .. } => true,
            Compression::Snappy { .. } => true,
            Compression::None => false,
        }
    }

    /// Get the compression threshold of the Compression.
    #[inline]
    pub fn threshold(&self) -> u16 {
        match self {
            Compression::Zlib { threshold, .. } => *threshold,
            Compression::Snappy { threshold, .. } => *threshold,
            Compression::None => 0x0000,
        }
    }

    /// Compress the given uncompressed src stream into the given dst stream
    /// with the compressed data
    #[inline]
    pub fn compress(
        &self,
        src: &[u8],
        dst: &mut Vec<u8>,
    ) -> Result<(), CompressionError> {
        match self {
            Compression::Zlib {
                threshold: _,
                compression_level,
            } => {
                let mut encoder = flate2::write::DeflateEncoder::new(
                    dst,
                    flate2::Compression::new(*compression_level as u32),
                );

                encoder.write_all(src).map_err(|e| CompressionError::ZlibError(Arc::new(e)))
            }
            Compression::Snappy { .. } => {
                let mut encoder = snap::write::FrameEncoder::new(dst);

                encoder.write_all(src).map_err(|e| CompressionError::SnappyError(Arc::new(e)))
            }
            Compression::None => {
                // unnecessary copying, this fn shouldn't be called when `compression_needed` returns false
                dst.write_all(src).map_err(|e| CompressionError::IOError(Arc::new(e)))
            }
        }
    }

    /// Decompress the given compressed src stream into the given dst stream
    /// with the decompressed data
    #[inline]
    pub fn decompress(
        &self,
        src: &[u8],
        dst: &mut Vec<u8>,
    ) -> Result<(), CompressionError> {
        match self {
            Compression::Zlib { .. } => {
                let mut decoder = flate2::read::DeflateDecoder::new(src);

                match io::copy(&mut decoder, dst) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(CompressionError::ZlibError(Arc::new(e))),
                }
            }
            Compression::Snappy { .. } => {
                let mut decoder = snap::read::FrameDecoder::new(src);

                match io::copy(&mut decoder, dst) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(CompressionError::SnappyError(Arc::new(e))),
                }
            }
            Compression::None => {
                // unnecessary copying, this fn shouldn't be called when `compression_needed` returns false
                match dst.write_all(src) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(CompressionError::IOError(Arc::new(e))),
                }
            }
        }
    }
}
