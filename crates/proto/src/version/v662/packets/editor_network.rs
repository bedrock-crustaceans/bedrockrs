use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::CompoundTag;

#[gamepacket(id = 190)]
#[derive(ProtoCodec)]
pub struct EditorNetworkPacket {
    pub binary_payload: CompoundTag,
}