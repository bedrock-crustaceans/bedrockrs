use bedrockrs_core::int::{VAR, LE};
use crate::types::chunk_pos::ChunkPos;
use bedrockrs_proto_core::ProtoCodec;

#[derive(Debug, Clone)]
pub struct LevelChunkPacket {
    pub chunk_position: ChunkPos,
    pub dimension_id: VAR<i32>,
    pub sub_chunk_count: VAR<i32>,
    pub cache_enabled: bool,
    pub serialized_chunk_data: Vec<u8>,

    pub client_needs_to_request_subchunks: bool,
    pub client_request_subchunk_limit: VAR<i32>
}

impl ProtoCodec for LevelChunkPacket {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), bedrockrs_proto_core::error::ProtoCodecError> {
        self.chunk_position.proto_serialize(stream)?;
        self.dimension_id.proto_serialize(stream)?;

        if !self.client_needs_to_request_subchunks {
            self.sub_chunk_count.proto_serialize(stream)?;
        }
        else {
            if !(self.client_request_subchunk_limit.into_inner() < 0) {
                VAR::<u32>::new(0xFFFFFFFE).proto_serialize(stream)?;
                self.client_request_subchunk_limit.proto_serialize(stream)?;
            }
            else {
                VAR::<u32>::new(0xFFFFFFFF).proto_serialize(stream)?;
            }
        }

        self.cache_enabled.proto_serialize(stream)?;
        if self.cache_enabled {
            // todo: implement sending with cached blobs.
            unimplemented!()
        }

        stream.extend_from_slice(&self.serialized_chunk_data);

        return Ok(());
    }

    fn proto_deserialize(stream: &mut std::io::Cursor<&[u8]>) -> Result<Self, bedrockrs_proto_core::error::ProtoCodecError> {
        unreachable!()
    }
}