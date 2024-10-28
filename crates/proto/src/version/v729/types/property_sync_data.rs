use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct PropertySyncData {
    pub int: IntEntriesList,
    pub float: FloatEntriesList,
}

#[derive(ProtoCodec, Debug, Clone)]
pub struct IntEntriesList {
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub entries: Vec<IntEntry>,
}

#[derive(ProtoCodec, Debug, Clone)]
pub struct IntEntry {
    #[endianness(var)]
    pub property_index: u32,
    #[endianness(var)]
    pub data: i32,
}

#[derive(ProtoCodec, Debug, Clone)]
pub struct FloatEntriesList {
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub entries: Vec<FloatEntry>,
}

#[derive(ProtoCodec, Debug, Clone)]
pub struct FloatEntry {
    #[endianness(var)]
    pub property_index: u32,
    #[endianness(le)]
    pub data: f32,
}
