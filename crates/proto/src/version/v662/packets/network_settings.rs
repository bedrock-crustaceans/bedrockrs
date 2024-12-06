use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::PacketCompressionAlgorithm;

#[gamepacket(id = 143)]
#[derive(ProtoCodec)]
pub struct NetworkSettingsPacket {
    #[endianness(le)]
    pub compression_threshold: u16,
    pub compression_algorithm: PacketCompressionAlgorithm,
    pub client_throttle_enabled: bool,
    pub client_throttle_threshold: i8,
    #[endianness(le)]
    pub client_throttle_scalar: f32,
}