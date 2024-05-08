use crate::compression::CompressionMethod;
use crate::error::CompressionError;

pub struct NoCompression {}

impl CompressionMethod for NoCompression {
    const ID_u8: u8 = 0xFF;
    const ID_u16: u16 = 0xFFFF;

    #[inline(always)]
    fn get_threshold(&self) -> u16 {
        u16::MAX
    }

    #[inline(always)]
    fn compress(&self, data: Vec<u8>) -> Result<Vec<u8>, CompressionError> {
        Ok(data)
    }

    #[inline(always)]
    fn decompress(&self, data: Vec<u8>) -> Result<Vec<u8>, CompressionError> {
        Ok(data)
    }
}
