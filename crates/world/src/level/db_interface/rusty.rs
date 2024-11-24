use crate::level::db_interface::bedrock_key::ChunkKey;
use crate::level::file_interface::RawWorldTrait;
use bedrockrs_core::Vec2;
use bedrockrs_shared::world::dimension::Dimension;
use miniz_oxide::deflate::{compress_to_vec, compress_to_vec_zlib, CompressionLevel};
use miniz_oxide::inflate::{decompress_to_vec, decompress_to_vec_zlib};
use rusty_leveldb::compressor::NoneCompressor;
use rusty_leveldb::{Compressor, CompressorList, Options, DB};
use std::collections::HashSet;
use std::marker::PhantomData;
use std::rc::Rc;

struct ZlibCompressor(u8);

impl ZlibCompressor {
    pub fn new(level: u8) -> Self {
        assert!(level <= 10);
        Self(level)
    }
}

impl Compressor for ZlibCompressor {
    fn encode(&self, block: Vec<u8>) -> rusty_leveldb::Result<Vec<u8>> {
        Ok(compress_to_vec_zlib(&block, self.0))
    }

    fn decode(&self, block: Vec<u8>) -> rusty_leveldb::Result<Vec<u8>> {
        decompress_to_vec_zlib(&block).map_err(|e| rusty_leveldb::Status {
            code: rusty_leveldb::StatusCode::CompressionError,
            err: e.to_string(),
        })
    }
}

struct RawZlibCompressor(u8);

impl RawZlibCompressor {
    pub fn new(level: u8) -> Self {
        assert!(level <= 10);
        Self(level)
    }
}
impl Compressor for RawZlibCompressor {
    fn encode(&self, block: Vec<u8>) -> rusty_leveldb::Result<Vec<u8>> {
        Ok(compress_to_vec(&block, self.0))
    }

    fn decode(&self, block: Vec<u8>) -> rusty_leveldb::Result<Vec<u8>> {
        decompress_to_vec(&block).map_err(|e| rusty_leveldb::Status {
            code: rusty_leveldb::StatusCode::CompressionError,
            err: e.to_string(),
        })
    }
}

pub fn mcpe_options(compression_level: u8) -> Options {
    let mut opt = Options::default();
    let mut list = CompressorList::new();
    list.set_with_id(0, NoneCompressor {});
    list.set_with_id(2, ZlibCompressor::new(compression_level));
    list.set_with_id(4, RawZlibCompressor::new(compression_level));
    opt.compressor_list = Rc::new(list);
    opt.compressor = 4;
    opt
}

pub struct RustyDBInterface<UserState> {
    db: DB,
    phantom_data: PhantomData<UserState>,
}

impl<UserState> RawWorldTrait for RustyDBInterface<UserState> {
    type Err = ();
    type UserState = UserState;

    fn set_subchunk_raw(
        &mut self,
        chunk_info: ChunkKey,
        chunk_bytes: &[u8],
        state: &mut Self::UserState,
    ) -> Result<(), Self::Err> {
        todo!()
    }

    fn get_subchunk_raw(
        &mut self,
        chunk_info: ChunkKey,
        state: &mut Self::UserState,
    ) -> Result<Option<Vec<u8>>, Self::Err> {
        todo!()
    }

    fn chunk_exists(
        &mut self,
        chunk_info: ChunkKey,
        state: &mut Self::UserState,
    ) -> Result<bool, Self::Err> {
        todo!()
    }

    fn write_subchunk_batch(
        &mut self,
        subchunk_batch_info: Vec<(ChunkKey, Vec<u8>)>,
        state: &mut Self::UserState,
    ) -> Result<(), Self::Err> {
        todo!()
    }

    fn write_subchunk_marker_batch(
        &mut self,
        subchunk_batch_info: Vec<ChunkKey>,
        state: &mut Self::UserState,
    ) -> Result<(), Self::Err> {
        todo!()
    }

    fn exist_chunk(
        &mut self,
        chunk_info: ChunkKey,
        state: &mut Self::UserState,
    ) -> Result<(), Self::Err> {
        todo!()
    }

    fn build_key(key: &ChunkKey) -> Vec<u8> {
        todo!()
    }

    fn new(
        path: &str,
        create_if_missing: bool,
        _: &mut Self::UserState,
    ) -> Result<Self, Self::Err> {
        todo!()
    }

    fn generated_chunks(
        &mut self,
        state: &mut Self::UserState,
    ) -> Result<HashSet<(Dimension, Vec2<i32>)>, Self::Err> {
        todo!()
    }
}
