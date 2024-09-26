use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct ResourcePacksStackPack {
    pub id: String,
    pub version: String,
    pub sub_pack_name: String,
}
