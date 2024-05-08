use std::io::Cursor;
use bedrock_core::types::i32be;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use serialize::error::{DeserilizationError, SerilizationError};
use serialize::proto::de::MCProtoDeserialize;
use serialize::proto::ser::MCProtoSerialize;

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

impl MCProtoSerialize for PlayStatusType {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError> where Self: Sized {
        i32be(match self.to_i32() {
            None => { return Err(SerilizationError::WriteIOError) }
            Some(v) => { v }
        }).proto_serialize(buf)
    }
}

impl MCProtoDeserialize for PlayStatusType {
    fn proto_deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError> where Self: Sized {
        match i32be::proto_deserialize(cursor) {
            Ok(v) => { match PlayStatusType::from_i32(v.0) {
                None => { Err(DeserilizationError::ReadIOError) }
                Some(v) => { Ok(v) }
            }}
            Err(e) => { return Err(e) }
        }
    }
}