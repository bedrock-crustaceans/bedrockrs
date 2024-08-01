use bedrockrs_core::int::{VAR, LE};
use crate::types::chunk_pos::ChunkPos;
use bedrockrs_proto_core::ProtoCodec;

#[derive(Debug, Clone)]
pub struct LevelChunkPacket {
    pub chunk_position: ChunkPos,
    pub dimension_id: VAR<i32>,
    pub sub_chunk_count: VAR<u32>,
    // not handling case where client requests subchunks..
    pub cache_enabled: bool,
    pub serialized_chunk_data: Vec<u8>
}

impl ProtoCodec for LevelChunkPacket {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), bedrockrs_proto_core::error::ProtoCodecError> {
        match self.chunk_position.proto_serialize(stream) {
            Ok(v) => {},
            Err(e) => return Err(e)
        };

        match self.dimension_id.proto_serialize(stream) {
            Ok(v) => {},
            Err(e) => return Err(e)
        }

        // No idea where this comes from, but its referenced in protocol docs with no explaination..
        const CLIENT_NEEDS_TO_REQUEST_SUBCHUNKS: bool = false;

        if CLIENT_NEEDS_TO_REQUEST_SUBCHUNKS {
            unimplemented!();
        }
        else {
            match self.sub_chunk_count.proto_serialize(stream) {
                Ok(v) => {},
                Err(e) => return Err(e)
            }
        }

        match self.cache_enabled.proto_serialize(stream) {
            Ok(v) => {},
            Err(e) => return Err(e)
        }

        if self.cache_enabled {
            // todo: send blobs
            unimplemented!();
        }

        stream.extend_from_slice(&self.serialized_chunk_data);

        return Ok(());
    }

    fn proto_deserialize(stream: &mut std::io::Cursor<&[u8]>) -> Result<Self, bedrockrs_proto_core::error::ProtoCodecError> {
        unreachable!()
    }
}