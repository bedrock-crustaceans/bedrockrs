use std::io::Cursor;

use bedrockrs_nbt::endian::little_endian_network::NbtLittleEndianNetwork;
use bedrockrs_nbt::NbtTag;

use crate::error::ProtoCodecError;
use crate::ProtoCodec;

impl ProtoCodec for NbtTag {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        self.nbt_serialize::<NbtLittleEndianNetwork>("", stream)
            .map_err(|e| ProtoCodecError::NbtError(e))
    }

    fn proto_deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        match NbtTag::nbt_deserialize::<NbtLittleEndianNetwork>(cursor) {
            Ok((_, v)) => Ok(v),
            Err(e) => Err(ProtoCodecError::NbtError(e)),
        }
    }
}
