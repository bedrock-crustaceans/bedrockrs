use crate::version::v662::types::ItemStackResponseInfo;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 148)]
#[derive(ProtoCodec)]
pub struct ItemStackResponsePacket {
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub responses: Vec<ItemStackResponseInfo>
}