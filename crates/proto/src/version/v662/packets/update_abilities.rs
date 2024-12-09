use crate::version::v662::types::SerializedAbilitiesData;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 187)]
#[derive(ProtoCodec)]
pub struct UpdateAbilitiesPacket {
    pub data: SerializedAbilitiesData,
}