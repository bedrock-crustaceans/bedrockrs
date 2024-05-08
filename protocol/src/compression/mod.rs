use crate::compression::snappy::SnappyCompression;
use crate::compression::zlib::ZlibCompression;
use crate::error::CompressionError;

pub mod none;
pub mod snappy;
pub mod zlib;

pub enum CompressionMethods {
    Zlib(ZlibCompression),
    Snappy(SnappyCompression),
    None,
}

pub trait CompressionMethod {
    /// Used after the raknet game packet header id for identifying with which
    /// CompressionMethod a packet was compressed
    const ID_u8: u8;
    /// Used in the NetworkSettingsPacket to identify which
    /// CompressionMethod should be used
    const ID_u16: u16;
    /// Get the compression threshold of the CompressionMethod.
    fn get_threshold(&self) -> u16;

    /// Compress the given data and return an owned Vector
    /// with the compressed data
    fn compress(&self, data: Vec<u8>) -> Result<Vec<u8>, CompressionError>;
    /// Decompress the given compressed data and return an owned Vector
    /// with the decompressed data
    fn decompress(&self, data: Vec<u8>) -> Result<Vec<u8>, CompressionError>;
}
