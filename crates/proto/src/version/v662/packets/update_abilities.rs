use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::SerializedAbilitiesData;

#[gamepacket(id = 187)]
#[derive(ProtoCodec)]
pub struct UpdateAbilitiesPacket {
    pub data: SerializedAbilitiesData,
}