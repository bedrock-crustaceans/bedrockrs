use bedrockrs_macros::ProtoCodec;
use crate::types::block_definition::BlockDefinition;

#[derive(Debug, Clone, ProtoCodec)]
pub struct BlockEntry {
    pub name: String,
    #[nbt]
    pub definition: BlockDefinition,
}
