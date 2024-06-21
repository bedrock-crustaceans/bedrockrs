use bedrock_core::*;
use proto_derive::ProtoCodec;

#[derive(Debug, Clone, ProtoCodec)]
pub struct NetworkSettingsPacket {
    /// Determines the smallest size of raw network payload to compress.
    /// - 0 is "disable compression"
    /// - 1 is "compress everything 1 byte or larger" (so everything)
    /// - others are just the normal threshold
    pub compression_threshold: LE<u16>,
    /// Determines the compression Algorithm used
    /// - 0x0000 is Zlib
    /// - 0x0001 is Snappy
    /// - 0xFFFF is No compression
    /// All 3 compression Algorithms are supported
    pub compression_algorithm: LE<u16>,
    pub client_throttle_enabled: bool,
    pub client_throttle_threshold: u8,
    pub client_throttle_scalar: LE<f32>,
}
