use crate::compression::CompressionMethod;
use crate::error::CompressionError;

pub struct SnappyCompression {
    pub threshold: u16,
}

impl CompressionMethod for SnappyCompression {
    const ID_u8: u8 = 0x01;
    const ID_u16: u16 = 0x0001;

    #[inline(always)]
    fn get_threshold(&self) -> u16 {
        self.threshold
    }

    #[inline(always)]
    fn compress(&self, data: Vec<u8>) -> Result<Vec<u8>, CompressionError> {
        let mut encoder = snap::raw::Encoder::new();

        match encoder.compress_vec(&*data) {
            Ok(v) => Ok(v),
            Err(e) => Err(CompressionError::SnappyError(e)),
        }
    }

    #[inline(always)]
    fn decompress(&self, data: Vec<u8>) -> Result<Vec<u8>, CompressionError> {
        let mut decoder = snap::raw::Decoder::new();

        match decoder.decompress_vec(&*data) {
            Ok(v) => Ok(v),
            Err(e) => Err(CompressionError::SnappyError(e)),
        }
    }
}
