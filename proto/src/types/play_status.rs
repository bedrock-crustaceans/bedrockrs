use std::io::Cursor;

use bedrock_core::i32be;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use proto_core::error::ProtoCodecError;
use proto_core::ProtoCodec;

#[derive(Debug, Copy, Clone, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum PlayStatusType {
    /// Sent after Login has been successfully decoded and the player has logged in
    LoginSuccess = 0,
    /// Displays "Could not connect: Outdated client!"
    FailedClient = 1,
    /// Displays "Could not connect: Outdated server!"
    FailedServer = 2,
    /// Sent after world data to spawn the player
    PlayerSpawn = 3,
    /// Displays "Unable to connect to world. Your school does not have access to this server."
    FailedInvalidTenant = 4,
    /// Displays "The server is not running Minecraft: Education Edition. Failed to connect."
    FailedVanillaEdu = 5,
    /// Displays "The server is running an incompatible edition of Minecraft. Failed to connect."
    FailedIncompatible = 6,
    /// Displays "Wow this server is popular! Check back later to see if space opens up. Server Full"
    FailedServerFull = 7,
}

impl ProtoCodec for PlayStatusType {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
        where
            Self: Sized,
    {
        match self.to_i32() {
            None => {
                return Err(ProtoCodecError::InvalidEnumID);
            }
            Some(v) => i32be(v).proto_serialize(buf),
        }
    }

    fn proto_deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, ProtoCodecError>
        where
            Self: Sized,
    {
        match i32be::proto_deserialize(cursor) {
            Ok(v) => match PlayStatusType::from_i32(v.0) {
                None => Err(ProtoCodecError::InvalidEnumID),
                Some(v) => Ok(v),
            },
            Err(e) => return Err(e),
        }
    }
}
