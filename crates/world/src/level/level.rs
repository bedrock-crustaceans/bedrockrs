use crate::level::chunk::LevelChunkTrait;
use crate::level::chunk_cache::SubchunkCacheKey;
use crate::level::db_interface::bedrock_key::ChunkKey;
use crate::level::file_interface::RawWorldTrait;
use crate::level::sub_chunk::{SubChunkDecoder, SubChunkTrait};
use crate::level::world_block::WorldBlockTrait;
use crate::level_try;
use crate::types::binary::BinaryBuffer;
use crate::types::clear_cache::ClearCacheContainer;
use bedrockrs_core::Vec2;
use bedrockrs_shared::world::dimension::Dimension;
use std::collections::hash_set::Iter;
use std::collections::HashSet;
use std::fmt::Debug;
use std::marker::PhantomData;
use thiserror::Error;

/// This is used when filtering chunks.
/// `ChunkSelectionFilter::Dimension` is used to just check if the dimension is the same.
/// `ChunkSelectionFilter::Filter` is used to perform more complex logic on the chunk to detect if it should be included
pub enum ChunkSelectionFilter {
    Dimension(Dimension),
    Filter(Box<dyn FnMut(Dimension, Vec2<i32>) -> bool>),
}

/// This is used when filtering subchunks.
///
/// `SubchunkSelectionFilter::Dimension` is used to just check if the dimension is the same.
///
/// `SubchunkSelectionFilter::Filter`
/// is used to perform more complex logic on the chunk to detect if it should be included
pub enum SubchunkSelectionFilter {
    Dimension(Dimension),
    Filter(Box<dyn FnMut(Dimension, i8, Vec2<i32>) -> bool>),
}

impl ChunkSelectionFilter {
    pub fn poll(&mut self, chunk_dim: Dimension, pos: Vec2<i32>) -> bool {
        match self {
            ChunkSelectionFilter::Dimension(dim) => dim == &chunk_dim,
            ChunkSelectionFilter::Filter(func) => func(chunk_dim, pos),
        }
    }
}

#[derive(Error, Debug)]
pub enum LevelError<DataBaseError: Debug, SubChunkDecodeError: Debug, SubChunkError: Debug> {
    #[error(transparent)]
    DatabaseError(DataBaseError),
    #[error(transparent)]
    SubChunkDecodeError(SubChunkDecodeError),
    #[error(transparent)]
    SubChunkError(SubChunkError),
}

pub struct ClosedLevel;

#[allow(dead_code)]
pub struct Level<
    UserState,
    UserWorldInterface: RawWorldTrait<UserState = UserState>,
    UserBlockType: WorldBlockTrait<UserState = UserState>,
    UserSubChunkType: SubChunkTrait<UserState = UserState, BlockType = UserBlockType>,
    UserSubChunkDecoder: SubChunkDecoder<UserState = UserState, BlockType = UserBlockType>,
> {
    db: UserWorldInterface,
    state: UserState,
    rw_cache: bool,
    cached_sub_chunks: ClearCacheContainer<SubchunkCacheKey, UserSubChunkType>,
    chunk_existence: HashSet<(Dimension, Vec2<i32>)>,
    _block_type_marker: PhantomData<UserBlockType>,
    _decoder_marker: PhantomData<UserSubChunkDecoder>,
}

#[allow(dead_code)]
impl<
        UserState,
        UserWorldInterface: RawWorldTrait<UserState = UserState>,
        UserBlockType: WorldBlockTrait<UserState = UserState>,
        UserSubChunkType: SubChunkTrait<UserState = UserState, BlockType = UserBlockType>,
        UserSubChunkDecoder: SubChunkDecoder<UserState = UserState, BlockType = UserBlockType>,
    > Level<UserState, UserWorldInterface, UserBlockType, UserSubChunkType, UserSubChunkDecoder>
where
    <UserSubChunkType as SubChunkTrait>::Err: Debug,
    <UserSubChunkDecoder as SubChunkDecoder>::Err: Debug,
    <UserWorldInterface as RawWorldTrait>::Err: Debug,
{
    /// Simple function used to open the world
    pub fn open(
        path: &str,
        create_db_if_missing: bool,
        rw_cache: bool,
        mut state: UserState,
    ) -> Result<
        Self,
        LevelError<UserWorldInterface::Err, UserSubChunkDecoder::Err, UserSubChunkType::Err>,
    > {
        let db = level_try!(
            DatabaseError,
            UserWorldInterface::new(path, create_db_if_missing, &mut state)
        );
        let mut this = Self {
            db,
            state,
            rw_cache,
            cached_sub_chunks: ClearCacheContainer::with_threshold(1024),
            chunk_existence: HashSet::new(),
            _block_type_marker: PhantomData,
            _decoder_marker: PhantomData,
        };
        this.chunk_existence = level_try!(DatabaseError, this.db.generated_chunks(&mut this.state));
        Ok(this)
    }

    /// # Safety
    /// This function is marked as `unsafe` because it allows the caller to bypass the caching systems.
    /// If modifications are made directly to the underlying database, the cache may become desynchronized,
    /// potentially leading to inconsistent.
    ///
    /// # When Safe to Use
    /// It is safe to use this function if you can guarantee that no information held in the cache
    /// will be modified or invalidated by your changes.
    pub unsafe fn underlying_world_interface(&mut self) -> &mut UserWorldInterface {
        &mut self.db
    }

    /// Checks if a given chunk exists
    pub fn chunk_exists(&mut self, xz: Vec2<i32>, dimension: Dimension) -> bool {
        self.chunk_existence.contains(&(dimension, xz))
    }

    /// Must call before destruction
    fn close(mut self) -> ClosedLevel {
        self.cull().unwrap();
        ClosedLevel
    }

    /// Returns all chunks (in the form of its key) that exist in the world
    pub fn existence_chunks(&self) -> Iter<'_, (Dimension, Vec2<i32>)> {
        self.chunk_existence.iter()
    }

    /// Fetches all chunk keys that satisfy the filter's constraints
    pub fn get_chunk_keys(&mut self, mut filter: ChunkSelectionFilter) -> Vec<Vec2<i32>> {
        self.chunk_existence
            .iter()
            .filter_map(|(chunk_dim, pos)| {
                if filter.poll(chunk_dim.clone(), *pos) {
                    Some(*pos)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Fetches all chunks that satisfy the filter
    pub fn get_chunks<T: LevelChunkTrait<Self, UserLevel = Self>>(
        &mut self,
        mut filter: ChunkSelectionFilter,
        min_max: Vec2<i8>,
    ) -> Result<Vec<T>, T::Err> {
        let positions: Vec<_> = self
            .chunk_existence
            .iter()
            .filter_map(|(chunk_dim, pos)| {
                if filter.poll(*chunk_dim, *pos) {
                    Some((*chunk_dim, *pos))
                } else {
                    None
                }
            })
            .collect();

        positions
            .into_iter()
            .map(|(dim, pos)| T::load_from_world(min_max, pos, dim, self))
            .collect()
    }

    /// Fetches a subchunk at a given xyz and dimension
    pub fn get_sub_chunk(
        &mut self,
        xz: Vec2<i32>,
        y: i8,
        dim: Dimension,
    ) -> Result<
        UserSubChunkType,
        LevelError<UserWorldInterface::Err, UserSubChunkDecoder::Err, UserSubChunkType::Err>,
    > {
        if self.rw_cache {
            if let Some(chunk) = self
                .cached_sub_chunks
                .get(&SubchunkCacheKey::new(xz, y, dim))
            {
                return Ok(chunk.state_clone(&mut self.state));
            }
        }
        let raw_bytes = level_try!(
            DatabaseError,
            self.db
                .get_subchunk_raw(ChunkKey::new_subchunk(xz, dim, y), &mut self.state)
        );
        let out = match raw_bytes {
            None => Ok::<
                (i8, Option<UserSubChunkType>),
                LevelError<
                    UserWorldInterface::Err,
                    UserSubChunkDecoder::Err,
                    UserSubChunkType::Err,
                >,
            >((y, None)),
            Some(bytes) => {
                if bytes.len() < 100 {
                    // This happens when there is no layers
                    let out = (y, None);
                    Ok(out)
                } else {
                    let mut bytes: BinaryBuffer = bytes.into();
                    let data = level_try!(
                        SubChunkDecodeError,
                        UserSubChunkDecoder::decode_bytes_as_chunk(&mut bytes, &mut self.state)
                    );
                    let out = (
                        y,
                        Some(level_try!(
                            SubChunkError,
                            UserSubChunkType::decode_from_raw(data, &mut self.state)
                        )),
                    );
                    Ok(out)
                }
            }
        }?;
        if self.rw_cache {
            if let Some(data) = &out.1 {
                let new = data.state_clone(&mut self.state);
                self.cached_sub_chunks
                    .insert(SubchunkCacheKey::new(xz, y, dim), new);
            }
        }
        if let None = &out.1 {
            Ok(UserSubChunkType::empty(out.0, self.state()))
        } else {
            Ok(out.1.unwrap())
        }
    }

    /// Sets a subchunk at the given xyz and dimension
    pub fn set_sub_chunk(
        &mut self,
        data: UserSubChunkType,
        xz: Vec2<i32>,
        y: i8,
        dim: Dimension,
    ) -> Result<
        (),
        LevelError<UserWorldInterface::Err, UserSubChunkDecoder::Err, UserSubChunkType::Err>,
    > {
        if self.rw_cache {
            self.cached_sub_chunks
                .insert(SubchunkCacheKey::new(xz, y, dim), data);
            level_try!(SubChunkDecodeError, self.perform_flush());
        } else {
            let raw = level_try!(
                SubChunkDecodeError,
                UserSubChunkDecoder::write_as_bytes(
                    level_try!(SubChunkError, data.to_raw(y, &mut self.state)),
                    false,
                    &mut self.state,
                )
            );
            let key = ChunkKey::new_subchunk(xz, dim, y);
            level_try!(
                DatabaseError,
                self.db.set_subchunk_raw(key, &raw, &mut self.state)
            );
            self.handle_exist(xz, dim);
        }
        Ok(())
    }

    /// Sets a whole chunk in the saved position of the chunk and the saved dimension.
    /// `xz_override` lets the xz position be replaced if copying the chunk
    /// `dim_override` lets the dimension of the chunk be changed if copying the chunk
    pub fn set_chunk<UserChunkType: LevelChunkTrait<Self, UserLevel = Self>>(
        &mut self,
        chnk: UserChunkType,
        xz_override: Option<Vec2<i32>>,
        dim_override: Option<Dimension>,
    ) -> Result<(), UserChunkType::Err> {
        chnk.write_to_world(self, xz_override, dim_override)
    }

    /// Fetches a chunk from the world at the given xz and dimension and with the given bounds
    /// ### Note:
    /// `min_max` is the min and max subchunks not blocks
    pub fn get_chunk<
        UserChunkType: LevelChunkTrait<
            Self,
            UserLevel = Self,
            Err = LevelError<
                UserWorldInterface::Err,
                UserSubChunkDecoder::Err,
                UserSubChunkType::Err,
            >,
        >,
    >(
        &mut self,
        min_max: Vec2<i8>,
        xz: Vec2<i32>,
        dim: Dimension,
    ) -> Result<UserChunkType, UserChunkType::Err> {
        UserChunkType::load_from_world(min_max, xz, dim, self)
    }

    fn handle_exist(&mut self, xz: Vec2<i32>, dim: Dimension) {
        self.chunk_existence.insert((dim, xz));
    }

    fn perform_flush(&mut self) -> Result<(), UserSubChunkDecoder::Err> {
        let mut batch_info: Vec<(ChunkKey, Vec<u8>)> = Vec::new();
        let mut exist_info: Vec<ChunkKey> = Vec::new();
        self.cached_sub_chunks.cull(|user_key, data| {
            let raw = UserSubChunkDecoder::write_as_bytes(
                data.to_raw(user_key.y, &mut self.state).unwrap(),
                false,
                &mut self.state,
            )?;
            let key = ChunkKey::new_subchunk(user_key.xz, user_key.dim, user_key.y);

            batch_info.push((key, raw));
            exist_info.push(ChunkKey::chunk_marker(user_key.xz, user_key.dim));
            Ok(())
        })?;
        if !batch_info.is_empty() {
            self.db
                .write_subchunk_batch(batch_info, &mut self.state)
                .unwrap()
        }
        if !exist_info.is_empty() {
            self.db
                .write_subchunk_marker_batch(exist_info, &mut self.state)
                .unwrap()
        }
        Ok(())
    }

    fn cull(&mut self) -> Result<(), UserSubChunkDecoder::Err> {
        let mut batch_info: Vec<(ChunkKey, Vec<u8>)> = Vec::new();
        let mut exist_info: Vec<ChunkKey> = Vec::new();
        self.cached_sub_chunks.clear(|user_key, data| {
            let raw = UserSubChunkDecoder::write_as_bytes(
                data.to_raw(user_key.y, &mut self.state).unwrap(),
                false,
                &mut self.state,
            )?;
            let key = ChunkKey::new_subchunk(user_key.xz, user_key.dim, user_key.y);

            batch_info.push((key, raw));
            exist_info.push(ChunkKey::chunk_marker(user_key.xz, user_key.dim));
            Ok(())
        })?;
        if !batch_info.is_empty() {
            self.db
                .write_subchunk_batch(batch_info, &mut self.state)
                .unwrap()
        }
        if !exist_info.is_empty() {
            self.db
                .write_subchunk_marker_batch(exist_info, &mut self.state)
                .unwrap()
        }
        Ok(())
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
{
    fn drop(&mut self) {
        panic!("Undroppable Type! Please call close on this type");
    }
}

pub trait LevelModificationProvider {
    type UserState;
    type UserWorldInterface;
    type UserBlockType;
    type UserSubChunkType;
    type UserSubChunkDecoder;
    type Error;

    fn get_sub_chunk(
        &mut self,
        xz: Vec2<i32>,
        y: i8,
        dim: Dimension,
    ) -> Result<Self::UserSubChunkType, Self::Error>;

    fn set_subchunk(
        &mut self,
        xz: Vec2<i32>,
        y: i8,
        dim: Dimension,
        chnk: Self::UserSubChunkType,
    ) -> Result<(), Self::Error>;

    fn state(&mut self) -> &mut Self::UserState;
    fn chunk_exists(&mut self, xz: Vec2<i32>, dimension: Dimension) -> bool;
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
    <UserSubChunkType as SubChunkTrait>::Err: Debug,
    <UserSubChunkDecoder as SubChunkDecoder>::Err: Debug,
    <UserWorldInterface as RawWorldTrait>::Err: Debug,
{
    type UserState = UserState;
    type UserWorldInterface = UserWorldInterface;
    type UserBlockType = UserBlockType;
    type UserSubChunkType = UserSubChunkType;
    type UserSubChunkDecoder = UserSubChunkDecoder;
    type Error =
        LevelError<UserWorldInterface::Err, UserSubChunkDecoder::Err, UserSubChunkType::Err>;

    fn get_sub_chunk(
        &mut self,
        xz: Vec2<i32>,
        y: i8,
        dim: Dimension,
    ) -> Result<Self::UserSubChunkType, Self::Error> {
        self.get_sub_chunk(xz, y, dim)
    }

    fn set_subchunk(
        &mut self,
        xz: Vec2<i32>,
        y: i8,
        dim: Dimension,
        chnk: Self::UserSubChunkType,
    ) -> Result<(), Self::Error> {
        self.set_sub_chunk(chnk, xz, y, dim)
    }

    fn state(&mut self) -> &mut Self::UserState {
        &mut self.state
    }

    fn chunk_exists(&mut self, xz: Vec2<i32>, dimension: Dimension) -> bool {
        self.chunk_exists(xz, dimension)
    }
}

#[cfg(feature = "default-impl")]
pub mod default_impl {
    use super::*;
    use crate::level::chunk::default_impl::LevelChunk;
    use crate::level::db_interface::rusty::RustyDBInterface;
    use crate::level::sub_chunk::default_impl::{SubChunk, SubChunkDecoderImpl};
    use crate::level::world_block::default_impl::WorldBlock;

    pub struct BedrockState {}
    pub type RawInterface = RustyDBInterface<BedrockState>;
    pub type BedrockWorldBlock = WorldBlock<BedrockState>;
    pub type BedrockSubChunk = SubChunk<BedrockWorldBlock, BedrockState>;
    pub type BedrockSubChunkDecoder = SubChunkDecoderImpl<BedrockWorldBlock, BedrockState>;
    pub type BedrockLevel = Level<
        BedrockState,
        RawInterface,
        BedrockWorldBlock,
        BedrockSubChunk,
        BedrockSubChunkDecoder,
    >;
    pub type BedrockChunk = LevelChunk<BedrockState, BedrockSubChunk, BedrockLevel>;
}
