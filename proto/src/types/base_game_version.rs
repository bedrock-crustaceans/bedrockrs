use std::io::Cursor;

use proto_core::error::ProtoCodecError;
use proto_core::ProtoCodec;

#[derive(Debug)]
pub struct BaseGameVersion(pub String);

impl ProtoCodec for BaseGameVersion {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        self.0.proto_serialize(buf)
    }

    fn proto_deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        match String::proto_deserialize(cursor) {
            Ok(v) => Ok(Self(v)),
            Err(e) => Err(e),
        }
    }
}
