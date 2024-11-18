use crate::level::chunk_cache::{ChunkCacheKey, SubchunkCacheKey};
use crate::level::db_interface::bedrock_key::ChunkKey;
use crate::level::file_interface::DatabaseBatchHolder;
use crate::level::file_interface::RawWorldTrait;
use crate::level::sub_chunk::{SubChunkDecoder, SubChunkTrait};
use crate::level::world_block::WorldBlockTrait;
use crate::types::binary::BinaryBuffer;
use crate::types::clear_cache::ClearCacheContainer;
use bedrockrs_core::Vec2;
use bedrockrs_shared::world::dimension::Dimension;
use mojang_leveldb::error::DBError;
use std::collections::HashSet;
use std::fmt::Debug;
use std::marker::PhantomData;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LevelError {
    #[error("DB Error: {0}")]
    Database(#[from] DBError),

    #[error("Unknown: {0}")]
    Unknown(#[from] anyhow::Error),
}

#[allow(dead_code)]
pub struct Level<
    UserState,
    UserWorldInterface: RawWorldTrait<UserState = UserState>,
    UserBlockType: WorldBlockTrait<UserState = UserState>,
    UserSubChunkType: SubChunkTrait<UserState = UserState, BlockType = UserBlockType>,
    UserSubChunkDecoder: SubChunkDecoder<UserState = UserState, BlockType = UserBlockType>,
> where
    LevelError: From<<UserSubChunkDecoder as SubChunkDecoder>::Err>,
    LevelError: From<<UserSubChunkType as SubChunkTrait>::Err>,
    LevelError: From<<UserWorldInterface as RawWorldTrait>::Err>,
    <UserSubChunkType as SubChunkTrait>::Err: Debug,
    <UserSubChunkDecoder as SubChunkDecoder>::Err: Debug,
    <UserWorldInterface as RawWorldTrait>::Err: Debug,
    DBError: From<<UserWorldInterface as RawWorldTrait>::Err>,
{
    db: UserWorldInterface,
    state: UserState,
    rw_cache: bool,
    cached_sub_chunks: ClearCacheContainer<SubchunkCacheKey, UserSubChunkType>,
    chunk_existence: HashSet<ChunkCacheKey>,
    _block_type_marker: PhantomData<UserBlockType>,
    _decoder_marker: PhantomData<UserSubChunkDecoder>,
}

impl<
        UserState,
        UserWorldInterface: RawWorldTrait<UserState = UserState>,
        UserBlockType: WorldBlockTrait<UserState = UserState>,
        UserSubChunkType: SubChunkTrait<UserState = UserState, BlockType = UserBlockType>,
        UserSubChunkDecoder: SubChunkDecoder<UserState = UserState, BlockType = UserBlockType>,
        //UserChunkType: LevelChunkTrait<UserState, UserWorldInterface, UserBlockType, UserSubChunkType, UserSubChunkDecoder>
    > Level<UserState, UserWorldInterface, UserBlockType, UserSubChunkType, UserSubChunkDecoder>
where
    LevelError: From<<UserSubChunkDecoder as SubChunkDecoder>::Err>,
    LevelError: From<<UserSubChunkType as SubChunkTrait>::Err>,
    LevelError: From<<UserWorldInterface as RawWorldTrait>::Err>,
    <UserSubChunkType as SubChunkTrait>::Err: Debug,
    <UserSubChunkDecoder as SubChunkDecoder>::Err: Debug,
    <UserWorldInterface as RawWorldTrait>::Err: Debug,
    DBError: From<<UserWorldInterface as RawWorldTrait>::Err>,
{
    pub fn open(
        path: &str,
        create_db_if_missing: bool,
        rw_cache: bool,
        mut state: UserState,
    ) -> Result<Self, DBError> {
        let db = UserWorldInterface::new(path, create_db_if_missing, &mut state)?;
        let this = Self {
            db,
            state,
            rw_cache,
            cached_sub_chunks: ClearCacheContainer::with_threshold(8192),
            chunk_existence: HashSet::new(),
            _block_type_marker: PhantomData,
            _decoder_marker: PhantomData,
        };

        Ok(this)
    }

    pub fn chunk_exists(
        &mut self,
        xz: Vec2<i32>,
        dimension: Dimension,
    ) -> Result<bool, LevelError> {
        Ok(self
            .db
            .chunk_exists(ChunkKey::chunk_marker(xz, dimension), &mut self.state)?)
    }

    fn close_wrap(&mut self) {
        self.cull();
    }

    pub fn get_sub_chunk(
        &mut self,
        xz: Vec2<i32>,
        y: i8,
        dim: Dimension,
    ) -> Result<UserSubChunkType, LevelError> {
        if self.rw_cache {
            if let Some(chunk) = self
                .cached_sub_chunks
                .get(&SubchunkCacheKey::new(xz, y, dim))
            {
                return Ok(chunk.state_clone(&mut self.state));
            }
        }
        let raw_bytes = self
            .db
            .get_subchunk_raw(ChunkKey::new_subchunk(xz, dim, y), &mut self.state)?;
        let out = match raw_bytes {
            None => Ok(UserSubChunkType::empty(y, &mut self.state)),
            Some(bytes) => {
                let mut bytes: BinaryBuffer = bytes.into();
                let data = UserSubChunkDecoder::decode_bytes_as_chunk(&mut bytes, &mut self.state)?;
                UserSubChunkType::decode_from_raw(data, &mut self.state)
            }
        }?;
        if self.rw_cache {
            let new = out.state_clone(&mut self.state);
            self.cached_sub_chunks
                .insert(SubchunkCacheKey::new(xz, y, dim), new);
        }
        Ok(out)
    }

    pub fn set_sub_chunk(
        &mut self,
        data: UserSubChunkType,
        xz: Vec2<i32>,
        y: i8,
        dim: Dimension,
    ) -> Result<(), LevelError> {
        if self.rw_cache {
            self.cached_sub_chunks
                .insert(SubchunkCacheKey::new(xz, y, dim), data);
            self.perform_flush();
        } else {
            let raw = UserSubChunkDecoder::write_as_bytes(
                data.to_raw(y, &mut self.state)?,
                false,
                &mut self.state,
            )?;
            let key = ChunkKey::new_subchunk(xz, dim, y);
            self.db.set_subchunk_raw(key, &raw, &mut self.state)?;
            self.db
                .exist_chunk(ChunkKey::chunk_marker(xz, dim), &mut self.state)?;
        }
        Ok(())
    }

    //FIXME: Fix the fact perform_flush and cull both violate the rules of write_subchunk_batch by also writing exist keys with it
    // They also share the same function so it will be worth making the function a member func and just passing the pointer

    //TODO: Make cull/clear return a Result type to allow for nicer error handling

    fn perform_flush(&mut self) {
        let mut write_batch = Vec::new();
        self.cached_sub_chunks.cull(|user_key, data| {
            let raw = UserSubChunkDecoder::write_as_bytes(
                data.to_raw(user_key.y, &mut self.state)
                    .expect("Failed to dump to raw"),
                false,
                &mut self.state,
            )
            .expect("Failed to translate to bytes");
            let mut subchunk_key_info = UserWorldInterface::build_key(&ChunkKey::new_subchunk(
                user_key.xz,
                user_key.dim,
                user_key.y,
            ));
            let key_info_end = subchunk_key_info.len();
            subchunk_key_info.extend(raw);
            let len = subchunk_key_info.len();
            write_batch.push(DatabaseBatchHolder::new(
                subchunk_key_info,
                0..key_info_end,
                key_info_end..len,
            ));
            let mut chunk_key =
                UserWorldInterface::build_key(&ChunkKey::chunk_marker(user_key.xz, user_key.dim));
            let end = chunk_key.len();
            let len = chunk_key.len();
            chunk_key.extend([0]);
            write_batch.push(DatabaseBatchHolder::new(chunk_key, 0..end, end..len));
        });
        if write_batch.len() != 0 {
            self.db
                .write_subchunk_batch(write_batch, &mut self.state)
                .unwrap()
        }
    }

    fn cull(&mut self) {
        let mut write_batch = Vec::new();
        self.cached_sub_chunks.cull(|user_key, data| {
            let raw = UserSubChunkDecoder::write_as_bytes(
                data.to_raw(user_key.y, &mut self.state)
                    .expect("Failed to dump to raw"),
                false,
                &mut self.state,
            )
            .expect("Failed to translate to bytes");
            let mut subchunk_key_info = UserWorldInterface::build_key(&ChunkKey::new_subchunk(
                user_key.xz,
                user_key.dim,
                user_key.y,
            ));
            let key_info_end = subchunk_key_info.len();
            subchunk_key_info.extend(raw);
            let len = subchunk_key_info.len();
            write_batch.push(DatabaseBatchHolder::new(
                subchunk_key_info,
                0..key_info_end,
                key_info_end..len,
            ));
            let mut chunk_key =
                UserWorldInterface::build_key(&ChunkKey::chunk_marker(user_key.xz, user_key.dim));
            let end = chunk_key.len();
            let len = chunk_key.len();
            chunk_key.extend([0]);
            write_batch.push(DatabaseBatchHolder::new(chunk_key, 0..end, end..len));
        });
        self.db
            .write_subchunk_batch(write_batch, &mut self.state)
            .unwrap()
    }
}

impl<
        UserState,
        UserWorldInterface: RawWorldTrait<UserState = UserState>,
        UserBlockType: WorldBlockTrait<UserState = UserState>,
        UserSubChunkType: SubChunkTrait<UserState = UserState, BlockType = UserBlockType>,
        UserSubChunkDecoder: SubChunkDecoder<UserState = UserState, BlockType = UserBlockType>,
    > Drop
    for Level<UserState, UserWorldInterface, UserBlockType, UserSubChunkType, UserSubChunkDecoder>
where
    LevelError: From<<UserSubChunkDecoder as SubChunkDecoder>::Err>,
    LevelError: From<<UserSubChunkType as SubChunkTrait>::Err>,
    LevelError: From<<UserWorldInterface as RawWorldTrait>::Err>,
    <UserSubChunkType as SubChunkTrait>::Err: Debug,
    <UserSubChunkDecoder as SubChunkDecoder>::Err: Debug,
    <UserWorldInterface as RawWorldTrait>::Err: Debug,
    DBError: From<<UserWorldInterface as RawWorldTrait>::Err>,
{
    fn drop(&mut self) {
        self.close_wrap()
    }
}

pub trait LevelModificationProvider {
    type UserState;
    type UserWorldInterface;
    type UserBlockType;
    type UserSubChunkType;
    type UserSubChunkDecoder;

    fn get_sub_chunk(
        &mut self,
        xz: Vec2<i32>,
        y: i8,
        dim: Dimension,
    ) -> Result<Self::UserSubChunkType, LevelError>;
}

impl<
        UserState,
        UserWorldInterface: RawWorldTrait<UserState = UserState>,
        UserBlockType: WorldBlockTrait<UserState = UserState>,
        UserSubChunkType: SubChunkTrait<UserState = UserState, BlockType = UserBlockType>,
        UserSubChunkDecoder: SubChunkDecoder<UserState = UserState, BlockType = UserBlockType>,
    > LevelModificationProvider
    for Level<UserState, UserWorldInterface, UserBlockType, UserSubChunkType, UserSubChunkDecoder>
where
    LevelError: From<<UserSubChunkDecoder as SubChunkDecoder>::Err>,
    LevelError: From<<UserSubChunkType as SubChunkTrait>::Err>,
    LevelError: From<<UserWorldInterface as RawWorldTrait>::Err>,
    <UserSubChunkType as SubChunkTrait>::Err: Debug,
    <UserSubChunkDecoder as SubChunkDecoder>::Err: Debug,
    <UserWorldInterface as RawWorldTrait>::Err: Debug,
    DBError: From<<UserWorldInterface as RawWorldTrait>::Err>,
{
    type UserState = UserState;
    type UserWorldInterface = UserWorldInterface;
    type UserBlockType = UserBlockType;
    type UserSubChunkType = UserSubChunkType;
    type UserSubChunkDecoder = UserSubChunkDecoder;

    fn get_sub_chunk(
        &mut self,
        xz: Vec2<i32>,
        y: i8,
        dim: Dimension,
    ) -> Result<Self::UserSubChunkType, LevelError> {
        self.get_sub_chunk(xz, y, dim)
    }
}

#[cfg(feature = "default-impl")]
pub mod default_impl {
    use super::*;
    use crate::level::db_interface::db::LevelDBInterface;
    use crate::level::sub_chunk::default_impl::{SubChunk, SubChunkDecoderImpl};
    use crate::level::world_block::default_impl::WorldBlock;

    pub struct State {}
    pub type BedrockLevel = Level<
        State,
        LevelDBInterface<State>,
        WorldBlock<State>,
        SubChunk<WorldBlock<State>, State>,
        SubChunkDecoderImpl<WorldBlock<State>, State>,
    >;
}
