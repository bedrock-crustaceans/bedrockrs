use std::io::Cursor;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use serialize::error::{DeserilizationError, SerilizationError};
use serialize::proto::de::MCProtoDeserialize;
use serialize::proto::ser::MCProtoSerialize;

#[derive(Debug, Copy, Clone, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum ResourcePacksResponseStatus {
    None = 0,
    Refused = 1,
    SendPacks = 2,
    HaveAllPacks = 3,
    Completed = 4,
}

impl MCProtoSerialize for ResourcePacksResponseStatus {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
        where
            Self: Sized,
    {
        match match self.to_u8() {
            None => return Err(SerilizationError::WriteIOError),
            Some(v) => v,
        }
            .proto_serialize(buf)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

impl MCProtoDeserialize for ResourcePacksResponseStatus {
    fn proto_deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
        where
            Self: Sized,
    {
        match u8::proto_deserialize(cursor) {
            Ok(v) => match ResourcePacksResponseStatus::from_u8(v) {
                None => return Err(DeserilizationError::ReadIOError),
                Some(v) => Ok(v),
            },
            Err(e) => Err(e),
        }
    }
}
