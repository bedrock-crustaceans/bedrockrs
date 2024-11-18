use crate::level::db_interface::db::LevelDBKey;
use crate::level::db_interface::key_level::KeyTypeTag;
use crate::types::buffer_slide::SlideBuffer;
use bedrockrs_core::Vec2;
use bedrockrs_shared::world::dimension::Dimension;

pub struct ChunkKey {
    xz: Vec2<i32>,
    dim: Dimension,
    key_type: KeyTypeTag,
    y_index: Option<i8>,
}

impl ChunkKey {
    pub fn new_subchunk(xz: Vec2<i32>, dim: Dimension, y_index: i8) -> Self {
        Self {
            xz,
            dim,
            key_type: KeyTypeTag::SubChunkPrefix,
            y_index: Some(y_index),
        }
    }

    pub fn chunk_marker(xz: Vec2<i32>, dim: Dimension) -> Self {
        Self {
            xz,
            dim,
            key_type: KeyTypeTag::Version,
            y_index: None,
        }
    }
}

impl LevelDBKey for ChunkKey {
    fn estimate_size(&self) -> usize {
        let mut size = std::mem::size_of::<Vec2<i32>>();
        if self.dim != Dimension::Overworld {
            size += std::mem::size_of::<i32>();
        }
        size += 1; // For the key_type
        if let Some(_) = self.y_index {
            size += 1;
        }
        size
    }

    fn write_key(&self, buffer: &mut SlideBuffer<Vec<u8>>) {
        buffer.write(self.xz.x).write(self.xz.y);
        if self.dim != Dimension::Overworld {
            buffer.write(self.dim as i32);
        }
        buffer.write(self.key_type.to_byte());
        if let Some(val) = self.y_index {
            buffer.write(val);
        }
    }
}
