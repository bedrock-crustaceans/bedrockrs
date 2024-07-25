use std::io::Cursor;
use bedrockrs_core::{ActorRuntimeID, VAR};

use crate::error::ProtoCodecError;
use crate::ProtoCodec;

impl ProtoCodec for ActorRuntimeID {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        VAR::new(self.0).proto_serialize(stream)
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        Ok(Self(VAR::proto_deserialize(stream)?.into_inner()))
    }
}
