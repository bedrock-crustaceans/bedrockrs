use std::io::Cursor;

use bedrock_core::read::ByteStreamRead;
use bedrock_core::write::ByteStreamWrite;
use nbt::endian::little_endian_network::NbtLittleEndianNetwork;
use nbt::NbtTag;

use crate::error::ProtoCodecError;
use crate::ProtoCodec;

impl ProtoCodec for NbtTag {
    fn proto_serialize(&self, stream: &mut ByteStreamWrite) -> Result<(), ProtoCodecError> {
        self.nbt_serialize::<NbtLittleEndianNetwork>("", stream)
            .map_err(|e| ProtoCodecError::NbtError(e))
    }

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError> {
        let mut cursor = Cursor::new(stream.get_ref().as_slice());

        match NbtTag::nbt_deserialize::<NbtLittleEndianNetwork>(&mut cursor) {
            Ok((_, v)) => Ok(v),
            Err(e) => Err(ProtoCodecError::NbtError(e)),
        }
    }
}
