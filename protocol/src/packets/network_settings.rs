use bedrock_core::types::*;
use serialize_derive::{MCProtoDeserialize, MCProtoSerialize};

#[derive(Debug, Copy, Clone, MCProtoSerialize, MCProtoDeserialize)]
pub struct NetworkSettingsPacket {
    /// Determines the smallest size of raw network payload to compress.
    /// - 0 is disable compression
    /// - 1 is compress everything 1 byte or larger (so everything)
    pub compression_threshold: u16le,
    /// Determines the compression Algorythm used
    /// - 0x0000 is Zlib
    /// - 0x0001 is Snappy
    /// - 0xFFFF is No compression
    /// All 3 compression Algorythms are supported
    pub compression_algorythm: u16le,
    pub client_throttle_enabled: bool,
    pub client_throttle_threshold: u8,
    pub client_throttle_scalar: f32,
}
