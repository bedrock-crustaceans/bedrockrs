use crate::level::level::LevelModificationProvider;
use crate::level::sub_chunk::SubChunkTrait;
use crate::level::world_block::WorldBlockTrait;
use bedrockrs_core::{Vec2, Vec3};
use bedrockrs_shared::world::dimension::Dimension;
use std::fmt::Debug;
use thiserror::Error;

/// Specifies the type of filter used when filling a region in the world.
///
/// # Type Parameters
/// - `UserBlockType`: A block type that implements the `WorldBlockTrait`.
pub enum FillFilter<UserBlockType: WorldBlockTrait> {
    /// Fills the entire region unconditionally, overwriting all blocks.
    Blanket,

    /// Replaces only the blocks that match the specified type.
    ///
    /// # Parameters
    /// - `UserBlockType`: The block type to be replaced.
    Replace(UserBlockType),

    /// Avoids overwriting blocks that match the specified type.
    ///
    /// # Parameters
    /// - `UserBlockType`: The block type to avoid.
    Avoid(UserBlockType),

    /// Uses a custom precedence function to determine whether a block should be replaced.
    ///
    /// # Parameters
    /// - A boxed function with the following parameters:
    ///   - `&UserBlockType`: The current block being evaluated.
    ///   - `Vec3<u8>`: The local coordinates of the block within the chunk.
    ///   - `Vec2<i32>`: The world-space XZ coordinates of the chunk.
    ///   - `i8`: The subchunk y.
    ///
    /// # Returns
    /// - `bool`: `true` to allow replacing the block, `false` to skip it.
    Precedence(Box<dyn Fn(&UserBlockType, Vec3<u8>, Vec2<i32>, i8) -> bool>),
}

#[derive(Error, Debug)]
pub enum FillError {
    #[error("Attempted to fill Subchunk {0} and got none back")]
    MissingSubchunk(i8),
    #[error("Attempted to read block at x: {0}, y {1}, z: {2} and got None")]
    BlockIndexDidntReturn(u8, u8, u8),
}

pub trait LevelChunkTrait<UserLevel: LevelModificationProvider>: Sized
where
    <Self as LevelChunkTrait<UserLevel>>::UserSubchunk: SubChunkTrait,
    <Self as LevelChunkTrait<UserLevel>>::UserBlock: WorldBlockTrait,
    <UserLevel as LevelModificationProvider>::UserSubChunkType: SubChunkTrait,
    <<Self as LevelChunkTrait<UserLevel>>::UserSubchunk as SubChunkTrait>::BlockType:
        WorldBlockTrait,
{
    type UserLevel; // = UserLevel;
    type UserBlock; // = UserLevel::UserBlockType;
    type UserSubchunk; // = UserLevel::UserSubChunkType;
    type UserState; // = UserLevel::UserState;
    type Err;

    /// Loads the chunk from the world based on the specified min and max subchunk indices,
    /// XZ coordinates, and dimension.
    ///
    /// # Parameters
    /// - `min_max`: Minimum and maximum subchunk indices.
    /// - `xz`: The XZ coordinate of the chunk in the world.
    /// - `dim`: The dimension where the chunk resides.
    /// - `level`: The level data to load the chunk into.
    ///
    /// # Returns
    /// - `Ok(Self)`: The loaded chunk.
    /// - `Err(Self::Err)`: An error occurred during loading.
    fn load_from_world(
        min_max: Vec2<i8>,
        xz: Vec2<i32>,
        dim: Dimension,
        level: &mut Self::UserLevel,
    ) -> Result<Self, Self::Err>;

    /// Writes the chunk back into the world, optionally overriding its position and dimension.
    ///
    /// # Parameters
    /// - `level`: The level data where the chunk will be written.
    /// - `xz_override`: Optional override for the chunk's XZ coordinates.
    /// - `dim_override`: Optional override for the chunk's dimension.
    ///
    /// # Returns
    /// - `Ok(())`: Successfully written to the world.
    /// - `Err(Self::Err)`: An error occurred during writing.
    fn write_to_world(
        self,
        level: &mut Self::UserLevel,
        xz_override: Option<Vec2<i32>>,
        dim_override: Option<Dimension>,
    ) -> Result<(), Self::Err>;

    /// Retrieves a mutable reference to the block at the specified XZ and Y coordinates within the chunk.
    ///
    /// # Parameters
    /// - `xz`: The XZ coordinate of the block within the chunk.
    /// - `y`: The Y coordinate of the block.
    ///
    /// # Returns
    /// - `Some(&mut Self::UserBlock)`: A mutable reference to the block if it exists.
    /// - `None`: No block exists at the specified coordinates.
    fn get_block_at_mut(&mut self, xz: Vec2<u8>, y: i16) -> Option<&mut Self::UserBlock>;

    /// Retrieves an immutable reference to the block at the specified XZ and Y coordinates within the chunk.
    ///
    /// # Parameters
    /// - `xz`: The XZ coordinate of the block within the chunk.
    /// - `y`: The Y coordinate of the block.
    ///
    /// # Returns
    /// - `Some(&Self::UserBlock)`: An immutable reference to the block if it exists.
    /// - `None`: No block exists at the specified coordinates.
    fn get_block_at(&self, xz: Vec2<u8>, y: i16) -> Option<&Self::UserBlock>;

    /// Sets the block at the specified XZ and Y coordinates within the chunk.
    ///
    /// # Parameters
    /// - `block`: The block to set.
    /// - `xz`: The XZ coordinate of the block within the chunk.
    /// - `y`: The Y coordinate of the block.
    ///
    /// # Returns
    /// - `Ok(())`: Successfully set the block.
    /// - `Err(Self::UserSubchunk::Err)`: An error occurred while setting the block.
    fn set_block_at(
        &mut self,
        block: Self::UserBlock,
        xz: Vec2<u8>,
        y: i16,
    ) -> Result<(), <Self::UserSubchunk as SubChunkTrait>::Err>;

    /// Returns the minimum and maximum subchunk indices for this chunk.
    ///
    /// # Returns
    /// - `Vec2<i8>`: A vector containing the minimum and maximum subchunk indices.
    fn min_max(&self) -> Vec2<i8>;

    /// Retrieves an immutable reference to the subchunk at the specified Y index.
    ///
    /// # Parameters
    /// - `y`: The Y index of the subchunk.
    ///
    /// # Returns
    /// - `Some(&Self::UserSubchunk)`: An immutable reference to the subchunk if it exists.
    /// - `None`: No subchunk exists at the specified index.
    fn get_subchunk(&self, y: i8) -> Option<&Self::UserSubchunk>;

    /// Retrieves a mutable reference to the subchunk at the specified Y index.
    ///
    /// # Parameters
    /// - `y`: The Y index of the subchunk.
    ///
    /// # Returns
    /// - `Some(&mut Self::UserSubchunk)`: A mutable reference to the subchunk if it exists.
    /// - `None`: No subchunk exists at the specified index.
    fn get_subchunk_mut(&mut self, y: i8) -> Option<&mut Self::UserSubchunk>;

    /// Gets the position of the chunk in world space as XZ coordinates.
    ///
    /// # Returns
    /// - `Vec2<i32>`: The chunk's position in world space.
    fn pos(&self) -> Vec2<i32>;
}

#[cfg(feature = "default-impl")]
pub mod default_impl {
    use super::*;
    use crate::level::file_interface::RawWorldTrait;
    use crate::level::level::LevelError;
    use crate::level::sub_chunk::{SubChunkDecoder, SubChunkTrait};
    use crate::level::world_block::WorldBlockTrait;
    use std::marker::PhantomData;
    use std::mem::MaybeUninit;
    use std::ops::{Deref, DerefMut};
    use std::slice::{Iter, IterMut};
    use std::vec::Vec;

    #[allow(dead_code)]
    pub struct LevelChunk<
        UserState,
        UserSubChunkType,
        UserLevelInterface: LevelModificationProvider,
    > {
        bounds: Vec2<i8>,
        xz: Vec2<i32>,
        dim: Dimension,
        sections: Vec<UserSubChunkType>,
        phantom_data: PhantomData<UserState>,
        _phantom_data: PhantomData<UserLevelInterface>,
    }

    impl<UserState, UserSubChunkType, UserLevelInterface: LevelModificationProvider> Deref
        for LevelChunk<UserState, UserSubChunkType, UserLevelInterface>
    {
        type Target = Vec<UserSubChunkType>;
        fn deref(&self) -> &Self::Target {
            &self.sections
        }
    }

    impl<UserState, UserSubChunkType, UserLevelInterface: LevelModificationProvider> DerefMut
        for LevelChunk<UserState, UserSubChunkType, UserLevelInterface>
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.sections
        }
    }

    impl<
            UserState,
            UserBlockType: WorldBlockTrait<UserState = UserState>,
            UserSubChunkType: SubChunkTrait<UserState = UserState, BlockType = UserBlockType>,
            UserLevelInterface: LevelModificationProvider,
        > LevelChunk<UserState, UserSubChunkType, UserLevelInterface>
    {
        // This function computes the true index of a subchunk in the section array.
        // E.g., with a bounds of -4 to 20 subchunks in size if a raw_index of -4 is passed in it will perform
        // -4 + 4 equaling 0 meaning the true index of a subchunk at y -4 is 0; this logic extends to any y level
        // y 2 would do 2 + 4 meaning the true index of a subchunk at y 2 is 6
        fn true_index(min_max: Vec2<i8>, raw_index: i8) -> i8 {
            #[cfg(debug_assertions)]
            if raw_index < min_max.x || raw_index > min_max.y {
                panic!("Y out of bounds");
            }
            raw_index + min_max.x.abs()
        }

        // The need for this function comes from the fact that when subchunks are in -y the y index is inverted.
        // This means that 20 -9 20 is near the bottom of the subchunk were as 20 9 20 is near the top of the subchunk (this is assuming we are using world space coords).
        // The Y can be thought more of as an offset from the base.
        // The base in this context is 0 if the subchunk is positive y and 16 if the subchunk is negative
        fn real_y(raw: i8) -> i8 {
            if raw >= 0 {
                raw
            } else {
                16 + raw
                // This looks a little confusing so ill explain.
                // raw is a negative number, meaning 16 + raw is basically 16 - abs(raw)
            }
        }

        /// Returns an iterator over all subchunks in this chunk, ordered from low to high.
        ///
        /// # Returns
        /// - `Iter<'_, UserSubChunkType>`: An iterator over the subchunks.
        pub fn iter(&self) -> Iter<'_, UserSubChunkType> {
            self.sections.iter()
        }

        /// Returns a mutable iterator over all subchunks in this chunk, ordered from low to high.
        ///
        /// # Returns
        /// - `IterMut<'_, UserSubChunkType>`: A mutable iterator over the subchunks.
        pub fn iter_mut(&mut self) -> IterMut<'_, UserSubChunkType> {
            self.sections.iter_mut()
        }

        /// Fetches an immutable reference to the subchunk at the specified Y level.
        ///
        /// # Parameters
        /// - `idx`: The Y level of the desired subchunk.
        ///
        /// # Returns
        /// - `Some(&UserSubChunkType)`: An immutable reference to the subchunk if it exists.
        /// - `None`: No subchunk exists at the given Y level.
        pub fn get_subchunk(&self, idx: i8) -> Option<&UserSubChunkType> {
            self.sections
                .get(Self::true_index(self.bounds, idx) as usize)
        }

        /// Fetches a mutable reference to the subchunk at the specified Y level.
        ///
        /// # Parameters
        /// - `idx`: The Y level of the desired subchunk.
        ///
        /// # Returns
        /// - `Some(&mut UserSubChunkType)`: A mutable reference to the subchunk if it exists.
        /// - `None`: No subchunk exists at the given Y level.
        pub fn get_subchunk_mut(&mut self, idx: i8) -> Option<&mut UserSubChunkType> {
            self.sections
                .get_mut(Self::true_index(self.bounds, idx) as usize)
        }

        /// Sets the subchunk at the specified Y level.
        ///
        /// # Parameters
        /// - `y`: The Y level where the subchunk will be set.
        /// - `chnk`: The subchunk to set at the specified Y level.
        pub fn set_subchunk(&mut self, y: i8, mut chnk: UserSubChunkType) {
            chnk.set_y(y);
            self[Self::real_y(y) as usize] = chnk;
        }

        /// Creates a new chunk filled with empty subchunks.
        ///
        /// # Parameters
        /// - `xz`: The XZ coordinates of the chunk in world space.
        /// - `min_max`: The minimum and maximum subchunk indices.
        /// - `dim`: The dimension where the chunk resides.
        /// - `state`: The state object used to create empty subchunks.
        ///
        /// # Returns
        /// - `Self`: A new chunk with empty subchunks.
        pub fn empty(
            xz: Vec2<i32>,
            min_max: Vec2<i8>,
            dim: Dimension,
            state: &mut UserState,
        ) -> Self {
            let ret_subchunks = (min_max.x..min_max.y)
                .map(|y| UserSubChunkType::empty(y, state))
                .collect::<Vec<_>>();
            Self {
                xz,
                bounds: min_max,
                sections: ret_subchunks,
                dim,
                phantom_data: PhantomData,
                _phantom_data: PhantomData,
            }
        }
    }

    impl<
        UserState,
        UserBlockType: WorldBlockTrait<UserState = UserState>,
        UserSubChunkType: SubChunkTrait<UserState = UserState, BlockType = UserBlockType>,
        UserLevelInterface: LevelModificationProvider<
            UserBlockType = UserBlockType,
            UserState = UserState,
            UserSubChunkType = UserSubChunkType,
            Error = LevelError<<<UserLevelInterface as LevelModificationProvider>::UserWorldInterface as RawWorldTrait>::Err,
                <<UserLevelInterface as LevelModificationProvider>::UserSubChunkDecoder as SubChunkDecoder>::Err,
                <UserSubChunkType as SubChunkTrait>::Err,>,
        >,
    >  LevelChunk<UserState, UserSubChunkType, UserLevelInterface>
    where
        <UserLevelInterface as LevelModificationProvider>::UserWorldInterface: RawWorldTrait,
        <UserLevelInterface as LevelModificationProvider>::UserSubChunkDecoder: SubChunkDecoder,
        <UserLevelInterface as LevelModificationProvider>::UserBlockType: WorldBlockTrait,
        <UserSubChunkType as SubChunkTrait>::Err: Debug,
        <<UserLevelInterface as LevelModificationProvider>::UserWorldInterface as RawWorldTrait>::Err: Debug,
        <<UserLevelInterface as LevelModificationProvider>::UserSubChunkDecoder as SubChunkDecoder>::Err: Debug,
    {
        /// Fills the entire chunk with the specified block, applying the given `FillFilter`.
        ///
        /// # Parameters
        /// - `block`: The block to fill the chunk with.
        /// - `filter`: The filter defining the conditions for filling blocks.
        ///
        /// # Returns
        /// - `Ok(&mut Self)`: Successfully filled the chunk.
        /// - `Err(FillError)`: An error occurred, such as a missing subchunk or invalid block index.
        ///
        /// # Errors
        /// - `FillError::MissingSubchunk(y_level)`: Subchunk at the specified Y level is missing.
        /// - `FillError::BlockIndexDidntReturn(x, y, z)`: Block index is invalid or did not return a block.
        ///
        /// # Behavior
        /// Iterates through all blocks in all subchunks, applying the `FillFilter`:
        /// - `Blanket`: Fills all blocks unconditionally.
        /// - `Replace(mask)`: Replaces only blocks matching the given mask.
        /// - `Avoid(mask)`: Fills blocks that do not match the mask.
        /// - `Precedence(func)`: Uses a custom function to determine if a block should be replaced.

        pub fn fill_chunk(&mut self, block: UserBlockType, filter: FillFilter<UserBlockType>) -> Result<&mut Self, FillError> {
            let pos = self.pos();
            for y_level in self.bounds.x..self.bounds.y {
                let subchunk = self.get_subchunk_mut(y_level)
                    .ok_or(FillError::MissingSubchunk(y_level))?;
                for z in 0..16u8 {
                    for y in 0..16u8 {
                        for x in 0..16u8 {
                            let blk = subchunk
                                .get_block((x, y, z).into())
                                .ok_or(FillError::BlockIndexDidntReturn(x, y, z))?;
                            if match &filter {
                                FillFilter::Blanket => true,
                                FillFilter::Replace(mask) =>
                                    mask == blk,
                                FillFilter::Avoid(mask) => mask != blk,
                                FillFilter::Precedence(func) => {
                                    func(blk, (x, y, z).into(), pos, subchunk.get_y())
                                }
                            } {
                                subchunk.set_block((x, y, z).into(), block.clone()).unwrap()
                            }
                        }
                    }
                }
            }
            Ok(self)
        }
    }
    impl<
            UserState,
            UserBlockType: WorldBlockTrait<UserState = UserState>,
            UserSubChunkType: SubChunkTrait<UserState = UserState, BlockType = UserBlockType>,
            UserLevelInterface: LevelModificationProvider<
                UserBlockType = UserBlockType,
                UserState = UserState,
                UserSubChunkType = UserSubChunkType,
                Error = LevelError<<<UserLevelInterface as LevelModificationProvider>::UserWorldInterface as RawWorldTrait>::Err,
                    <<UserLevelInterface as LevelModificationProvider>::UserSubChunkDecoder as SubChunkDecoder>::Err,
                    <UserSubChunkType as SubChunkTrait>::Err,>,
            >,
        > LevelChunkTrait<UserLevelInterface> for LevelChunk<UserState, UserSubChunkType, UserLevelInterface>
    where
        <UserLevelInterface as LevelModificationProvider>::UserWorldInterface: RawWorldTrait,
        <UserLevelInterface as LevelModificationProvider>::UserSubChunkDecoder: SubChunkDecoder,
        <UserLevelInterface as LevelModificationProvider>::UserBlockType: WorldBlockTrait,
        <UserSubChunkType as SubChunkTrait>::Err: Debug,
        <<UserLevelInterface as LevelModificationProvider>::UserWorldInterface as RawWorldTrait>::Err: Debug,
        <<UserLevelInterface as LevelModificationProvider>::UserSubChunkDecoder as SubChunkDecoder>::Err: Debug,
    {
        type UserLevel = UserLevelInterface;
        type UserBlock = UserBlockType;
        type UserSubchunk = UserSubChunkType;
        type UserState = UserState;
        type Err = LevelError<
            <UserLevelInterface::UserWorldInterface as RawWorldTrait>::Err,
            <UserLevelInterface::UserSubChunkDecoder as SubChunkDecoder>::Err,
            <UserLevelInterface::UserSubChunkType as SubChunkTrait>::Err,
        >;

        fn load_from_world(
            min_max: Vec2<i8>,
            xz: Vec2<i32>,
            dim: Dimension,
            level: &mut Self::UserLevel,
        ) -> Result<Self, Self::Err> {
            let mut subchunk_list: Vec<MaybeUninit<UserSubChunkType>> = (min_max.x..min_max.y)
                .map(|_|  MaybeUninit::uninit() )
                .collect();
            for y in min_max.x..min_max.y {
                let subchunk = level.get_sub_chunk(xz, y, dim);
                match subchunk {
                    Ok(subchunk) => {
                        let idx = Self::true_index(min_max, y) as usize;
                        subchunk_list[idx].write(subchunk);
                    }
                    Err(err) => {
                        for r in min_max.x..y {
                            // Safety: We are only dropping subchunks which came before this one meaning they have to be init
                            unsafe {
                                subchunk_list[Self::true_index(min_max, r) as usize]
                                    .assume_init_drop();
                            }
                        }
                        return Err(err);
                    }
                }
            }
            // Safety: Since `MaybeUninit` is a ZST the ABI of the two types is the same
            Ok(Self {
                bounds: min_max,
                xz,
                dim,
                sections: unsafe { std::mem::transmute(subchunk_list) },
                phantom_data: PhantomData,
                _phantom_data: PhantomData
            })
        }

        fn write_to_world(
            self,
            level: &mut Self::UserLevel,
            xz_override: Option<Vec2<i32>>,
            dim_override: Option<Dimension>,
        ) -> Result<(), Self::Err> {
            for sub_chnk in self.sections {
                level.set_subchunk(
                    xz_override.unwrap_or(self.xz),
                    sub_chnk.get_y(),
                    dim_override.unwrap_or(self.dim),
                    sub_chnk,
                )?;
            }
            Ok(())
        }

        fn get_block_at_mut(&mut self, xz: Vec2<u8>, y: i16) -> Option<&mut Self::UserBlock> {
            self.sections
                .get_mut(Self::true_index(self.bounds, (y / 16) as i8) as usize)?
                .get_block_mut((xz.x, Self::real_y((y / 16) as i8) as u8, xz.y).into())
        }

        fn get_block_at(&self, xz: Vec2<u8>, y: i16) -> Option<&Self::UserBlock> {
            self.sections
                .get(Self::true_index(self.bounds, (y / 16) as i8) as usize)?
                .get_block((xz.x, Self::real_y((y / 16) as i8) as u8, xz.y).into())
        }

        fn set_block_at(
            &mut self,
            block: Self::UserBlock,
            xz: Vec2<u8>,
            y: i16,
        ) -> Result<(), <Self::UserSubchunk as SubChunkTrait>::Err> {
            self.sections
                .get_mut(Self::true_index(self.bounds, (y / 16) as i8) as usize)
                .unwrap() /*TODO: Figure out a way to report this error back to the user*/
                .set_block(
                    (xz.x, Self::real_y((y / 16) as i8) as u8, xz.y).into(),
                    block,
                )
        }

        fn min_max(&self) -> Vec2<i8> {
            self.bounds
        }

        fn get_subchunk(&self, y: i8) -> Option<&Self::UserSubchunk> {
            self.sections.get(Self::true_index(self.bounds, y) as usize)
        }

        fn get_subchunk_mut(&mut self, y: i8) -> Option<&mut Self::UserSubchunk> {
            self.sections
                .get_mut(Self::true_index(self.bounds, y) as usize)
        }

        fn pos(&self) -> Vec2<i32> {
            self.xz
        }


    }
}
