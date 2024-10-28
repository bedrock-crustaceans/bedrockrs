use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct ResourcePackInfoType {
    pub id: String,
    pub version: String,
    #[endianness(le)]
    pub size: u64,
    pub content_key: String,
    pub sub_pack_name: String,
    pub content_identify: String,
    pub has_scripts: bool,
    pub ray_tracing_capable: bool,
}
