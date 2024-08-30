use bedrockrs_core::{int::LE, Vec3};
use bedrockrs_proto_derive::{gamepacket, ProtoCodec};

#[gamepacket(id = 18)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ServerPlayerPostMovePositionPacket {
    pos: Vec3<LE<f32>>,
}
