use crate::level::db_interface::bedrock_key::ChunkKey;
use std::ops::Range;

pub struct DatabaseBatchHolder {
    collective: Vec<u8>,
    key_range: Range<usize>,
    data_range: Range<usize>,
}

impl DatabaseBatchHolder {
    pub fn new(collective: Vec<u8>, key_range: Range<usize>, data_range: Range<usize>) -> Self {
        Self {
            collective,
            key_range,
            data_range,
        }
    }

    pub fn key(&self) -> &[u8] {
        &self.collective[self.key_range.clone()]
    }

    pub fn data(&self) -> &[u8] {
        &self.collective[self.data_range.clone()]
    }
}

pub trait RawWorldTrait: Sized {
    type Err;

    type UserState;

    fn set_subchunk_raw(
        &mut self,
        chunk_info: ChunkKey,
        chunk_bytes: &[u8],
        state: &mut Self::UserState,
    ) -> Result<(), Self::Err>;

    fn get_subchunk_raw(
        &mut self,
        chunk_info: ChunkKey,
        state: &mut Self::UserState,
    ) -> Result<Option<Vec<u8>>, Self::Err>;

    fn chunk_exists(
        &mut self,
        chunk_info: ChunkKey,
        state: &mut Self::UserState,
    ) -> Result<bool, Self::Err>;

    fn write_subchunk_batch(
        &mut self,
        subchunk_batch_info: Vec<DatabaseBatchHolder>,
        state: &mut Self::UserState,
    ) -> Result<(), Self::Err>;

    fn exist_chunk(
        &mut self,
        chunk_info: ChunkKey,
        state: &mut Self::UserState,
    ) -> Result<(), Self::Err>;

    fn build_key(key: &ChunkKey) -> Vec<u8>;

    fn new(
        path: &str,
        create_if_missing: bool,
        state: &mut Self::UserState,
    ) -> Result<Self, Self::Err>;

    fn generated_chunks(&mut self, state: &mut Self::UserState)
        -> Result<Vec<ChunkKey>, Self::Err>;

}
