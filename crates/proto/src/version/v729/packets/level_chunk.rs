use crate::version::v729::types::chunk_pos::ChunkPos;
use bedrockrs_macros::gamepacket;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;
use bedrockrs_shared::world::dimension::Dimension;
use varint_rs::VarintWriter;

#[gamepacket(id = 58)]
#[derive(Debug, Clone)]
pub struct LevelChunkPacket {
    pub chunk_position: ChunkPos,
    pub dimension_id: Dimension,
    pub sub_chunk_count: u32,
    pub cache_enabled: bool,
    pub serialized_chunk_data: Vec<u8>,
    pub client_needs_to_request_subchunks: bool,
    pub client_request_subchunk_limit: i32,
}

impl ProtoCodec for LevelChunkPacket {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        self.chunk_position.proto_serialize(stream)?;
        self.dimension_id.proto_serialize(stream)?;

        if !self.client_needs_to_request_subchunks {
            stream.write_u32_varint(self.sub_chunk_count)?;
        } else if self.client_request_subchunk_limit >= 0 {
            stream.write_u32_varint(u32::MAX - 1)?;
            stream.write_i32_varint(self.client_request_subchunk_limit)?;
        } else {
            stream.write_u32_varint(u32::MAX)?;
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

    fn proto_deserialize(_stream: &mut std::io::Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        todo!()
    }

    fn get_size_prediction(&self) -> usize {
        // TODO
        1
    }
}
