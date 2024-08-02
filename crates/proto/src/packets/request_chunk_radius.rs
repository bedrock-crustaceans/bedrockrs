use bedrockrs_core::int::VAR;
use bedrockrs_proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct RequestChunkRadiusPacket {
    pub chunk_radius: VAR<u32>,
    pub chunk_radius_max: u8,
}
