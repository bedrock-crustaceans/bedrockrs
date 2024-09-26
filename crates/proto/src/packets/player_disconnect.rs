use std::io::Cursor;

use crate::types::disconnect_reason::DisconnectReason;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;
use bedrockrs_macros::gamepacket;

#[gamepacket(id = 5)]
#[derive(Debug, Clone)]
pub struct DisconnectPlayerPacket {
    /// Seems to have no effect on the message being shown.
    /// It is just for telemetry.
    pub reason: DisconnectReason,
    pub message: Option<String>,
}

// ProtoCodec
impl ProtoCodec for DisconnectPlayerPacket {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        self.reason.proto_serialize(buf)?;

        if let Some(text) = &self.message {
            bool::proto_serialize(&false, buf)?;
            text.proto_serialize(buf)?;
        } else {
            // Skip message
            bool::proto_serialize(&true, buf)?;
        }

        Ok(())
    }

    fn proto_deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        let reason = DisconnectReason::proto_deserialize(cursor)?;

        // Read if the message should be skipped
        let skip_message = bool::proto_deserialize(cursor)?;

        let message = match skip_message {
            true => None,
            false => Some(String::proto_deserialize(cursor)?),
        };

        Ok(Self { reason, message })
    }
}
