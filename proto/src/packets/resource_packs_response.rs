use bedrock_core::{LE, VAR};
use bedrock_core::read::ByteStreamRead;
use bedrock_core::write::ByteStreamWrite;
use proto_core::error::ProtoCodecError;
use proto_core::ProtoCodec;

use crate::types::resource_packs_response_status::ResourcePacksResponseStatus;

#[derive(Debug, Clone)]
pub struct ResourcePacksResponsePacket {
    pub response: ResourcePacksResponseStatus,
    /// The packs that are downloaded/getting downloaded
    /// with their pack name as strings
    pub downloading_packs: Vec<String>,
}

impl ProtoCodec for ResourcePacksResponsePacket {
    fn proto_serialize(&self, stream: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match self.response.proto_serialize(stream) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        let len = match self.downloading_packs.len().try_into() {
            Ok(v) => v,
            Err(e) => {
                return Err(ProtoCodecError::FromIntError(e));
            }
        };

        // Write length of downloading packs as an u16le
        match LE::<u16>::new(len).proto_serialize(stream) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        for downloading_pack in &self.downloading_packs {
            match downloading_pack.proto_serialize(stream) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }

    fn proto_deserialize(cursor: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        // Read the Response
        let response = match ResourcePacksResponseStatus::proto_deserialize(cursor) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        let downloading_packs_len = match VAR::<u16>::proto_deserialize(cursor) {
            Ok(v) => v.into_inner(),
            Err(e) => return Err(e),
        };

        let mut downloading_packs = vec![];

        for _ in 0..downloading_packs_len {
            match String::proto_deserialize(cursor) {
                Ok(v) => downloading_packs.push(v),
                Err(e) => return Err(e),
            }
        }

        Ok(Self {
            response,
            downloading_packs,
        })
    }
}
