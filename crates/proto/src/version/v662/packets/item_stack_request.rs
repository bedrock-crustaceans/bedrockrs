use crate::version::v662::enums::{ItemStackRequestActionType, TextProcessingEventOrigin};
use crate::version::v662::types::ItemStackRequestSlotInfo;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[derive(ProtoCodec, Clone, Debug)]
pub struct ActionsEntry {
    pub action_type: ItemStackRequestActionType,
    pub amount: i8,
    pub source: ItemStackRequestSlotInfo,
    pub destination: ItemStackRequestSlotInfo,
}

#[derive(ProtoCodec, Clone, Debug)]
pub struct RequestsEntry {
    #[endianness(var)]
    pub client_request_id: u32,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub actions: Vec<ActionsEntry>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub strings_to_filter: Vec<String>,
    pub strings_to_filter_origin: TextProcessingEventOrigin,
}

#[gamepacket(id = 147)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct ItemStackRequestPacket {
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub requests: Vec<RequestsEntry>,
}