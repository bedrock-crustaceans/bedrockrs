use bedrockrs_core::read::ByteStreamRead;
use bedrockrs_core::write::ByteStreamWrite;
use bedrockrs_core::VAR;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;

#[derive(Debug, Clone)]
pub struct DisconnectPacket {
    /// TODO: Convert into enum, if possible
    ///
    /// Seems to have no effect on the message being shown.
    /// It is just for telemetry.
    pub reason: VAR<i32>,
    pub message: Option<String>,
}

impl ProtoCodec for DisconnectPacket {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match self.reason.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        };

        match &self.message {
            // Skip message
            None => {
                match true.proto_serialize(buf) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
            // Don't skip message
            Some(str) => {
                match false.proto_serialize(buf) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                };

                match str.proto_serialize(buf) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
        }

        Ok(())
    }

    fn proto_deserialize(cursor: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        let reason = match VAR::<i32>::proto_deserialize(cursor) {
            Ok(v) => v,
            Err(e) => {
                return Err(e);
            }
        };

        let skip_message = match bool::proto_deserialize(cursor) {
            Ok(v) => v,
            Err(e) => {
                return Err(e);
            }
        };

        let message = match skip_message {
            true => None,
            false => match String::proto_deserialize(cursor) {
                Ok(v) => Some(v),
                Err(e) => {
                    return Err(e);
                }
            },
        };

        Ok(Self { reason, message })
    }
}
