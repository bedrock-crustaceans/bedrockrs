use std::io::Cursor;

use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, Copy, Clone, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum ResourcePacksResponseStatus {
    None = 0,
    Refused = 1,
    SendPacks = 2,
    HaveAllPacks = 3,
    Completed = 4,
}

impl ProtoCodec for ResourcePacksResponseStatus {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        let status = match self.to_u8() {
            Some(v) => v,
            None => {
                return Err(ProtoCodecError::InvalidEnumID);
            }
        };

        match status.proto_serialize(stream) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        match u8::proto_deserialize(stream) {
            Ok(v) => match ResourcePacksResponseStatus::from_u8(v) {
                None => return Err(ProtoCodecError::InvalidEnumID),
                Some(v) => Ok(v),
            },
            Err(e) => Err(e),
        }
    }
}
