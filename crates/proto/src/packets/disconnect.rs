use std::io::Cursor;

use crate::types::disconnect_reason::DisconnectReason;
use bedrockrs_core::int::VAR;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;

#[derive(Debug, Clone)]
pub struct DisconnectPacket {
    /// Seems to have no effect on the message being shown.
    /// It is just for telemetry.
    pub reason: DisconnectReason,
    pub message: Option<String>,
}

// ProtoCodec
impl ProtoCodec for DisconnectPacket {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        self.reason.proto_serialize(buf)?;

        match &self.message {
            // Skip message
            None => {
                bool::proto_serialize(&true, buf)?;
            }
            // Don't skip message
            Some(str) => {
                bool::proto_serialize(&false, buf)?;

                str.proto_serialize(buf)?;
            }
        }

        Ok(())
    }

    fn proto_deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        let reason = DisconnectReason::proto_deserialize(cursor)?;

        let skip_message = bool::proto_deserialize(cursor)?;

        let message = match skip_message {
            true => None,
            false => Some(String::proto_deserialize(cursor)?),
        };

        Ok(Self { reason, message })
    }
}
