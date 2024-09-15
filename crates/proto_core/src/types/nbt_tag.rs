use std::io::Cursor;

use crate::error::ProtoCodecError;
use crate::ProtoCodec;

impl ProtoCodec for nbtx::Value {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        nbtx::to_net_bytes_in(stream, self)?;
        Ok(())
    }

    fn proto_deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        let nbt = nbtx::from_net_bytes::<nbtx::Value, _>(cursor)?;
        Ok(nbt)
    }
}
