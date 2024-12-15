use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::ItemUseMethod;

#[gamepacket(id = 142)]
#[derive(ProtoCodec)]
pub struct CompletedUsingItemPacket {
    #[endianness(le)]
    pub item_id: u16,
    pub item_use_method: ItemUseMethod,
}