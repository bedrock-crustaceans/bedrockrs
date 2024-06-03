use std::io::Cursor;
use bedrock_core::read::ByteStreamRead;

use bedrock_core::u16le;
use bedrock_core::write::ByteStreamWrite;
use proto_core::error::ProtoCodecError;
use proto_core::ProtoCodec;

use crate::types::resource_packs_response_status::ResourcePacksResponseStatus;

#[derive(Debug)]
pub struct ResourcePacksResponsePacket {
    pub response: ResourcePacksResponseStatus,
    /// The packs that are downloaded/getting downloaded
    /// with their pack name as strings
    pub downloading_packs: Vec<String>,
}

impl ProtoCodec for ResourcePacksResponsePacket {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match self.response.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        match u16le(self.downloading_packs.len() as u16).proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        for downloading_pack in &self.downloading_packs {
            match downloading_pack.proto_serialize(buf) {
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

        let downloading_packs_len = match u16le::proto_deserialize(cursor) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        let mut downloading_packs = vec![];

        for _ in 0..downloading_packs_len.0 {
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
