use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::ItemStackResponseInfo;

#[gamepacket(id = 148)]
#[derive(ProtoCodec)]
pub struct ItemStackResponsePacket {
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub responses: Vec<ItemStackResponseInfo>
}