use bedrockrs_core::int::{LE, VAR};
use bedrockrs_proto_derive::ProtoCodec;

#[derive(ProtoCodec,Debug, Clone)]
struct IntEntry {
    index: VAR<u32>,
    data: VAR<i32>,
}
#[derive(ProtoCodec,Debug, Clone)]
struct IntEntriesList {
    #[len_repr(VAR::<u32>)]
    entries: Vec<IntEntry>,
}

#[derive(ProtoCodec,Debug, Clone)]
struct FloatEntry {
    index: VAR<u32>,
    data: LE<f32>,
}

#[derive(ProtoCodec,Debug, Clone)]
struct FloatEntriesList {
    #[len_repr(VAR::<u32>)]
    entries: Vec<FloatEntry>,
}

#[derive(Debug, Clone)]
pub enum PropertySyncData {
    Int(IntEntriesList),
    Float(FloatEntriesList),
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
        Ok(Self::Int(IntEntriesList::proto_deserialize(stream)?))
    }
}
