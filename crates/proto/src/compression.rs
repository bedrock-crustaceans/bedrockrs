use crate::error::CompressionError;
use flate2::{write::DeflateEncoder, read::DeflateDecoder};
use flate2::Compression as CompressionLevel;
use snap::{write::FrameEncoder as SnapEncoder, read::FrameDecoder as SnapDecoder};
use std::io::{Read, Write};

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
            Compression::None => u8::MAX,
        }
    }

    /// Used in the NetworkSettingsPacket to identify which
    /// CompressionMethod should be used
    #[inline]
    pub const fn id_u16(&self) -> u16 {
        match self {
            Compression::Zlib { .. } => 0x0000,
            Compression::Snappy { .. } => 0x0001,
            Compression::None => u16::MAX,
        }
    }

    /// Specifies if functions like [`Self::compress`] and [`Self::decompress`] need to be used.
    /// This is necessary for optimizing compression.
    #[inline]
    pub const fn needed(&self) -> bool {
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
            Compression::None => 0,
        }
    }

    /// Compress the given uncompressed src stream into the given dst stream
    /// with the compressed data
    #[inline]
    pub fn compress(&self, src: Vec<u8>) -> Result<Vec<u8>, CompressionError> {
        let buf = match self {
            Compression::Zlib {
                threshold,
                compression_level,
            } => {
                if *threshold as usize >= src.len() {
                    src
                } else {
                    let dst = Vec::with_capacity(src.len() + 1);

                    let mut encoder =
                        DeflateEncoder::new(dst, CompressionLevel::new(*compression_level as u32));

                    encoder
                        .write_all(src.as_slice())
                        .map_err(|err| CompressionError::ZlibError(Box::new(err)))?;

                    encoder
                        .finish()
                        .map_err(|err| CompressionError::ZlibError(Box::new(err)))?
                }
            }
            Compression::Snappy { threshold } => {
                if *threshold as usize >= src.len() {
                    src
                } else {
                    let dst = Vec::with_capacity(src.len());

                    let mut encoder = SnapEncoder::new(dst);

                    encoder
                        .write_all(src.as_slice())
                        .map_err(|e| CompressionError::SnappyError(e))?;

                    encoder
                        .into_inner()
                        .map_err(|err| CompressionError::SnappyError(err.into_error()))?
                }
            }
            Compression::None => src,
        };

        Ok(buf)
    }

    /// Decompress the given compressed src stream into the given dst stream
    /// with the decompressed data
    #[inline]
    pub fn decompress(&self, src: Vec<u8>) -> Result<Vec<u8>, CompressionError> {
        let buf = match self {
            Compression::Zlib { .. } => {
                let mut dst = Vec::with_capacity(src.len());
                
                let mut decoder = DeflateDecoder::new(src.as_slice());
                decoder.read_to_end(&mut dst)?;
                
                dst
            }
            Compression::Snappy { .. } => {
                let mut dst = Vec::with_capacity(src.len());
                
                let mut decoder = SnapDecoder::new(src.as_slice());
                decoder.read_to_end(&mut dst)?;
                
                dst
            }
            Compression::None => src,
        };
        
        Ok(buf)
    }
}
