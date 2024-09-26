use crate::types::chunk_pos::ChunkPos;
use bedrockrs_core::int::{LE, VAR};
use bedrockrs_macros::gamepacket;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;

#[gamepacket(id = 58)]
#[derive(Debug, Clone)]
pub struct LevelChunkPacket {
    pub chunk_position: ChunkPos,
    pub dimension_id: VAR<i32>,
    pub sub_chunk_count: VAR<u32>,
    pub cache_enabled: bool,
    pub serialized_chunk_data: Vec<u8>,
    pub client_needs_to_request_subchunks: bool,
    pub client_request_subchunk_limit: VAR<i32>,
}

impl ProtoCodec for LevelChunkPacket {
    fn proto_serialize(
        &self,
        stream: &mut Vec<u8>,
    ) -> Result<(), bedrockrs_proto_core::error::ProtoCodecError> {
        self.chunk_position.proto_serialize(stream)?;
        self.dimension_id.proto_serialize(stream)?;

        if !self.client_needs_to_request_subchunks {
            self.sub_chunk_count.proto_serialize(stream)?;
        } else {
            if !(self.client_request_subchunk_limit.into_inner() < 0) {
                VAR::<u32>::new(u32::MAX - 1).proto_serialize(stream)?;
                self.client_request_subchunk_limit.proto_serialize(stream)?;
            } else {
                VAR::<u32>::new(u32::MAX).proto_serialize(stream)?;
            }
        }

        self.cache_enabled.proto_serialize(stream)?;
        if self.cache_enabled {
            // todo: implement sending with cached blobs.
            unimplemented!()
        }

        let len = self
            .serialized_chunk_data
            .len()
            .try_into()
            .map_err(ProtoCodecError::FromIntError)?;

        VAR::<u32>::new(len).proto_serialize(stream)?;

        stream.extend_from_slice(&self.serialized_chunk_data);

        println!("finish");

        Ok(())
    }

    fn proto_deserialize(
        stream: &mut std::io::Cursor<&[u8]>,
    ) -> Result<Self, bedrockrs_proto_core::error::ProtoCodecError> {
        unreachable!()
    }
}
