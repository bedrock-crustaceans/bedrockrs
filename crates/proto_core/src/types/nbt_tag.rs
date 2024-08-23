use std::io::Cursor;

use bedrockrs_nbt as nbt;

use crate::error::ProtoCodecError;
use crate::ProtoCodec;

impl ProtoCodec for nbt::Value {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        nbt::to_var_bytes_in(stream, self)?;
        Ok(())
    }

    fn proto_deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        todo!("Deserialize to concrete type instead of NbtTag");
    }
}
