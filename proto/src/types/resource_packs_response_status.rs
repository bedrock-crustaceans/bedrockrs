use bedrockrs_core::stream::read::ByteStreamRead;
use bedrockrs_core::stream::write::ByteStreamWrite;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;

#[derive(Debug, Copy, Clone, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum ResourcePacksResponseStatus {
    None = 0,
    Refused = 1,
    SendPacks = 2,
    HaveAllPacks = 3,
    Completed = 4,
}

impl ProtoCodec for ResourcePacksResponseStatus {
    fn proto_serialize(&self, stream: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
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

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
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
