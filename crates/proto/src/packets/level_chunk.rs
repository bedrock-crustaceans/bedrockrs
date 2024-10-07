use crate::types::chunk_pos::ChunkPos;
use bedrockrs_macros::gamepacket;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;
use varint_rs::VarintWriter;

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
    ) -> Result<(), ProtoCodecError> {
        self.chunk_position.proto_serialize(stream)?;
        self.dimension_id.proto_serialize(stream)?;

        if !self.client_needs_to_request_subchunks {
            self.sub_chunk_count.proto_serialize(stream)?;
        } else {
            if !(self.client_request_subchunk_limit.into_inner() < 0) {
                stream.write_u32_varint(u32::MAX - 1)?;
                self.client_request_subchunk_limit.proto_serialize(stream)?;
            } else {
                stream.write_u32_varint(u32::MAX)?;
            }
        }

        self.cache_enabled.proto_serialize(stream)?;
        if self.cache_enabled {
            todo!("implement sending with cached blobs.")
        }

        let len = self.serialized_chunk_data.len().try_into()?;

        stream.write_u32_varint(len)?;
        stream.extend_from_slice(&self.serialized_chunk_data);

        println!("finish");

        Ok(())
    }

    fn proto_deserialize(
        stream: &mut std::io::Cursor<&[u8]>,
    ) -> Result<Self, bedrockrs_proto_core::error::ProtoCodecError> {
        todo!()
    }

    fn get_size_prediction(&self) -> usize {
        // TODO
        1
    }
}
