use serialize_derive::{MCProtoDeserialize, MCProtoSerialize};

#[derive(Debug, MCProtoSerialize, MCProtoDeserialize)]
pub struct ResourcePacksStackPack {
    pub id: String,
    pub version: String,
    pub sub_pack_name: String,
}