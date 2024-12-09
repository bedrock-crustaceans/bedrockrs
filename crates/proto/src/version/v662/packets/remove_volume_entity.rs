use crate::version::v662::types::EntityNetID;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 167)]
#[derive(ProtoCodec)]
pub struct RemoveVolumeEntityPacket {
    pub entity_network_id: EntityNetID,
    #[endianness(var)]
    pub dimension_type: i32,
}