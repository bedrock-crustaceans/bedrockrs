use bedrockrs_core::int::VAR;
use bedrockrs_proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct RequestChunkRadiusPacket {
    chunk_radius: VAR<u32>,
    chunk_radius_max: u8,
}
