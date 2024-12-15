use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 106)]
#[derive(ProtoCodec)]
pub struct RemoveObjectivePacket {
    pub objective_name: String,
}