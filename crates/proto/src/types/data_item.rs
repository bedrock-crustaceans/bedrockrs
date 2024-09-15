use bedrockrs_core::{
    int::{LE, VAR},
    Vec3,
};
use bedrockrs_proto_core::ProtoCodec;
use bedrockrs_proto_macros::ProtoCodec;
use std::collections::HashMap;

#[derive(ProtoCodec, Debug, Clone)]
pub struct DataItem {
    pub id: VAR<u32>,
    pub value: DataItemValue,
}

#[derive(Debug, Clone)]
pub enum DataItemValue {
    ValByte(u8),
    ValShort(i16),
    ValInt(i32),
    ValFloat(f32),
    ValString(String),
    ValCompoundTag(HashMap<String, nbtx::Value>),
    ValPos(Vec3<VAR<i32>>),
    ValInt64(LE<i64>),
    ValVec3(Vec3<LE<f32>>),
}

impl ProtoCodec for DataItemValue {
    fn proto_serialize(
        &self,
        stream: &mut Vec<u8>,
    ) -> Result<(), bedrockrs_proto_core::error::ProtoCodecError> {
        unimplemented!()
    }

    fn proto_deserialize(
        stream: &mut std::io::Cursor<&[u8]>,
    ) -> Result<Self, bedrockrs_proto_core::error::ProtoCodecError> {
        unimplemented!()
    }
}
