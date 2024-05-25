use std::io::Cursor;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use proto_core::error::ProtoCodecError;
use proto_core::ProtoCodec;

#[derive(Debug, Copy, Clone, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum ResourcePacksResponseStatus {
    None = 0,
    Refused = 1,
    SendPacks = 2,
    HaveAllPacks = 3,
    Completed = 4,
}

impl ProtoCodec for ResourcePacksResponseStatus {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match match self.to_u8() {
            None => return Err(ProtoCodecError::InvalidEnumID),
            Some(v) => v,
        }
            .proto_serialize(buf)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn proto_deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        match u8::proto_deserialize(cursor) {
            Ok(v) => match ResourcePacksResponseStatus::from_u8(v) {
                None => return Err(ProtoCodecError::InvalidEnumID),
                Some(v) => Ok(v),
            },
            Err(e) => Err(e),
        }
    }
}
