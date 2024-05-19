use std::io::Cursor;
use bedrock_core::types::ivar32;
use serialize::error::{DeserilizationError, SerilizationError};
use serialize::proto::de::MCProtoDeserialize;
use serialize::proto::ser::MCProtoSerialize;

#[derive(Debug)]
pub struct DisconnectPacket {
    /// TODO: Convert into enum, if possible
    /// Seems to have no effect on the message being shown.
    /// It is just for telemetry.
    pub reason: ivar32,
    pub message: Option<String>,
}

impl MCProtoSerialize for DisconnectPacket {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError> where Self: Sized {
        match self.reason.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => { return Err(e) }
        };

        match &self.message {
            // Skip message
            None => {
                match true.proto_serialize(buf) {
                    Ok(_) => {}
                    Err(e) => { return Err(e) }
                };
            }
            // Don't skip message
            Some(str) => {
                match false.proto_serialize(buf) {
                    Ok(_) => {}
                    Err(e) => { return Err(e) }
                };

                match str.proto_serialize(buf) {
                    Ok(_) => {}
                    Err(e) => { return Err(e) }
                };
            }
        }

        Ok(())
    }
}

impl MCProtoDeserialize for DisconnectPacket {
    fn proto_deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError> where Self: Sized {
        let reason = match ivar32::proto_deserialize(cursor){
            Ok(v) => { v }
            Err(e) => { return Err(e) }
        };

        let skip_message = match bool::proto_deserialize(cursor){
            Ok(v) => { v }
            Err(e) => { return Err(e) }
        };

        let message = match skip_message {
            true => {
                None
            }
            false => {
                match String::proto_deserialize(cursor) {
                    Ok(v) => { Some(v) }
                    Err(e) => { return Err(e) }
                }
            }
        };

        Ok(Self {
            reason,
            message
        })
    }
}