use std::io::prelude::*;
use std::io::Write;

use flate2::write::DeflateDecoder;
use flate2::Compression;

use crate::compression::CompressionMethod;
use crate::error::CompressionError;

pub struct ZlibCompression {
    pub threshold: u16,
    /// Needs to be a number between 0 and 9.
    /// Indicitaes how compressed the data becomes.
    ///
    /// - 0 = None
    /// - 1 = Fastest
    /// - 6 = Default
    /// - 9 = Best
    pub compression_level: u8,
}

impl CompressionMethod for ZlibCompression {
    const ID_u8: u8 = 0x00;
    const ID_u16: u16 = 0x0000;

    #[inline(always)]
    fn get_threshold(&self) -> u16 {
        self.threshold
    }

    #[inline(always)]
    fn compress(&self, data: Vec<u8>) -> Result<Vec<u8>, CompressionError> {
        let mut encoder = flate2::write::DeflateEncoder::new(
            Vec::new(),
            Compression::new(self.compression_level as u32),
        );

        match encoder.write_all(&*data) {
            Ok(_) => {}
            Err(e) => return Err(CompressionError::ZlibError(Box::new(e))),
        }

        match encoder.finish() {
            Ok(v) => Ok(v),
            Err(e) => Err(CompressionError::ZlibError(Box::new(e))),
        }
    }

    #[inline(always)]
    fn decompress(&self, data: Vec<u8>) -> Result<Vec<u8>, CompressionError> {
        let buf = Vec::new();
        let mut decoder = DeflateDecoder::new(buf);

        match decoder.write_all(&*data) {
            Ok(_) => {}
            Err(e) => return Err(CompressionError::ZlibError(Box::new(e))),
        }

        match decoder.finish() {
            Ok(v) => Ok(v),
            Err(e) => Err(CompressionError::ZlibError(Box::new(e))),
        }
    }
}
