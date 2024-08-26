use bedrockrs_core::int::{LE, VAR};

#[derive(ProtoCodec, Debug, Clone)]
pub struct PropertySyncData {
    pub int: IntEntriesList,
    pub float: FloatEntriesList,
}

use bedrockrs_proto_derive::ProtoCodec;
#[derive(ProtoCodec, Debug, Clone)]
pub struct IntEntriesList {
    #[len_repr(VAR::<u32>)]
    pub entries: Vec<IntEntry>,
}

#[derive(ProtoCodec, Debug, Clone)]
pub struct FloatEntriesList {
    #[len_repr(VAR::<u32>)]
    pub entries: Vec<FloatEntry>,
}

#[derive(ProtoCodec, Debug, Clone)]
pub struct IntEntry {
    pub property_index: VAR<u32>,
    pub data: VAR<i32>,
}

#[derive(ProtoCodec, Debug, Clone)]
pub struct FloatEntry {
    pub property_index: VAR<u32>,
    pub data: LE<f32>,
}
