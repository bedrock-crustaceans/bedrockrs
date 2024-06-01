#[allow(non_upper_case_globals)]
use crate::compression::CompressionMethod;
use crate::error::CompressionError;

pub struct NoCompression {}

impl CompressionMethod for NoCompression {
    const ID_u8: u8 = 0xFF;
    const ID_u16: u16 = 0xFFFF;

    #[inline]
    fn get_threshold(&self) -> u16 {
        u16::MAX
    }

    #[inline]
    fn compress(&self, data: Vec<u8>) -> Result<Vec<u8>, CompressionError> {
        Ok(data)
    }

    #[inline]
    fn decompress(&self, data: Vec<u8>) -> Result<Vec<u8>, CompressionError> {
        Ok(data)
    }
}
