use bedrockrs_core::Vec2;
use bedrockrs_shared::world::dimension::Dimension;

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct SubchunkCacheKey {
    pub xz: Vec2<i32>,
    pub y: i8,
    pub dim: Dimension,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct ChunkCacheKey {
    xz: Vec2<i32>,
    sequence_id: usize,
}

impl SubchunkCacheKey {
    pub fn new(xz: Vec2<i32>, y: i8, dim: Dimension) -> Self {
        Self { xz, y, dim }
    }
}

impl ChunkCacheKey {
    pub fn new(xz: Vec2<i32>, sequence_id: usize) -> Self {
        Self { xz, sequence_id }
    }
}
