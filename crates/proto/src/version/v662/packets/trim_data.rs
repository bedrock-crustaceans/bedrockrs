use bedrockrs_macros::{gamepacket, ProtoCodec};

#[derive(ProtoCodec)]
struct TrimPattern {
    pub item_name: String,
    pub pattern_id: String,
}

#[derive(ProtoCodec)]
struct TrimMaterial {
    pub material_id: String,
    pub color: String,
    pub item_name: String,
}

#[gamepacket(id = 302)]
#[derive(ProtoCodec)]
pub struct TrimDataPacket {
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub trim_pattern_list: Vec<TrimPattern>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub trim_material_list: Vec<TrimMaterial>,
}