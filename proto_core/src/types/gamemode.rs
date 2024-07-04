use std::sync::Arc;

use bedrock_core::gamemode::Gamemode;
use bedrock_core::read::ByteStreamRead;
use bedrock_core::write::ByteStreamWrite;
use bedrock_core::VAR;

use crate::error::ProtoCodecError;
use crate::ProtoCodec;

impl ProtoCodec for Gamemode {
    fn proto_serialize(&self, stream: &mut ByteStreamWrite) -> Result<(), ProtoCodecError> {
        let int = match self {
            Gamemode::Survival => 0x00,
            Gamemode::Creative => 0x01,
            Gamemode::Adventure => 0x02,
            Gamemode::SurvivalSpectator => 0x03,
            Gamemode::CreativeSpectator => 0x04,
            Gamemode::Default => 0x05,
            Gamemode::Spectator => 0x06,
        };

        match VAR::<i32>::new(int).write(stream) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(Arc::new(e))),
        }
    }

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError> {
        todo!()
    }
}
