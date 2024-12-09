use crate::version::v662::enums::ItemStackNetResult;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub struct ItemStackResponseInfo {
    pub result: ItemStackNetResult,
    #[endianness(var)]
    pub client_request_id: i32,
    
}