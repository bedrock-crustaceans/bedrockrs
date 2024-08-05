use bedrockrs_core::int::{LE, VAR};
use bedrockrs_proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct IntEntry {
    pub index: VAR<u32>,
    pub data: VAR<i32>,
}
#[derive(ProtoCodec, Debug, Clone)]
pub struct IntEntriesList {
    #[len_repr(VAR::<u32>)]
    pub entries: Vec<IntEntry>,
}

#[derive(ProtoCodec, Debug, Clone)]
pub struct FloatEntry {
    pub index: VAR<u32>,
    pub data: LE<f32>,
}

#[derive(ProtoCodec, Debug, Clone)]
pub struct FloatEntriesList {
    #[len_repr(VAR::<u32>)]
    pub entries: Vec<FloatEntry>,
}

#[derive(Debug, Clone)]
pub struct PropertySyncData {
    pub int: IntEntriesList,
    pub float: FloatEntriesList,
}

use bedrockrs_proto_core::ProtoCodec;
impl ProtoCodec for PropertySyncData {
    fn proto_serialize(
        &self,
        stream: &mut Vec<u8>,
    ) -> Result<(), bedrockrs_proto_core::error::ProtoCodecError> {
        Ok(())
    }

    fn proto_deserialize(
        stream: &mut std::io::Cursor<&[u8]>,
    ) -> Result<Self, bedrockrs_proto_core::error::ProtoCodecError> {
        unimplemented!()
    }
}
