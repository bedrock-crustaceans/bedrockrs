use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 143)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct NetworkSettingsPacket {
    /// Determines the smallest size of raw network payload to compress.
    /// - 0 is "disable compression"
    /// - 1 is "compress everything 1 byte or larger" (so everything)
    /// - others are just the normal threshold
    #[endianness(le)]
    pub compression_threshold: u16,
    /// Determines the compression Algorithm used
    /// - 0x0000 is Zlib
    /// - 0x0001 is Snappy
    /// - 0xFFFF is No compression
    ///
    /// All three compression Algorithms are supported
    #[endianness(le)]
    pub compression_algorithm: u16,
    // TODO: Document what this field does
    pub client_throttle: bool,
    // TODO: Document what this field does
    pub client_throttle_threshold: u8,
    // TODO: Document what this field does
    #[endianness(le)]
    pub client_throttle_scalar: f32,
}
