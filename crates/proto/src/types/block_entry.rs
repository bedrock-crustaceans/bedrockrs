use crate::types::block_definition::BlockDefinition;
use bedrockrs_macros::ProtoCodec;

#[derive(Debug, Clone, ProtoCodec)]
pub struct BlockEntry {
    pub name: String,
    #[nbt]
    pub definition: BlockDefinition,
}
