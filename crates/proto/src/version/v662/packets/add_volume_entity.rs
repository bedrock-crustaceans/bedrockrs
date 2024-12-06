use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::{CompoundTag, EntityNetID, NetworkBlockPosition};

#[gamepacket(id = 166)]
#[derive(ProtoCodec)]
pub struct AddVolumeEntityPacket {
    pub entity_network_id: EntityNetID,
    pub components: CompoundTag,
    pub json_identifier: String,
    pub instance_name: String,
    pub min_bounds: NetworkBlockPosition,
    pub max_bounds: NetworkBlockPosition,
    #[endianness(var)]
    pub dimension_type: i32,
    pub engine_version: String,
}