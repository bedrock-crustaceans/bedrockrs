use bedrockrs_macros::ProtoCodec;
use crate::version::v662::enums::ItemStackNetResult;

#[derive(ProtoCodec)]
pub struct ItemStackResponseInfo {
    pub result: ItemStackNetResult,
    #[endianness(var)]
    pub client_request_id: i32,
    
}