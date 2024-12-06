use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::CompoundTag;

#[gamepacket(id = 122)]
#[derive(ProtoCodec)]
pub struct BiomeDefinitionListPacket {
    pub biome_definition_data: CompoundTag,
}