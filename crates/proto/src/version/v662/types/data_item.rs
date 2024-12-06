use bedrockrs_macros::ProtoCodec;
use crate::version::v662::enums::DataItemType;

#[derive(ProtoCodec)]
pub struct DataItem {
    #[endianness(var)]
    pub data_item_id: u32,
    pub data_item_type: DataItemType,
}