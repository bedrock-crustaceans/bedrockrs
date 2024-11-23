use crate::level::level::LevelModificationProvider;
use crate::level::sub_chunk::SubChunkTrait;
use bedrockrs_core::Vec2;
use bedrockrs_shared::world::dimension::Dimension;

pub trait LevelChunkTrait<UserLevel: LevelModificationProvider>: Sized
where
    <Self as LevelChunkTrait<UserLevel>>::UserSubchunk: SubChunkTrait,
    <UserLevel as LevelModificationProvider>::UserSubChunkType: SubChunkTrait,
{
    type UserLevel; // = UserLevel;
    type UserBlock; // = UserLevel::UserBlockType;
    type UserSubchunk; // = UserLevel::UserSubChunkType;
    type UserState; // = UserLevel::UserState;
    type Err;

    /// `min_max` is the min and max subchunk not block
    fn load_from_world(
        min_max: Vec2<i8>,
        xz: Vec2<i32>,
        dim: Dimension,
        level: &mut Self::UserLevel,
    ) -> Result<Self, Self::Err>;
    fn write_to_world(
        self,
        level: &mut Self::UserLevel,
        xz_override: Option<Vec2<i32>>,
        dim_override: Option<Dimension>,
    ) -> Result<(), Self::Err>;

    fn get_block_at_mut(&mut self, xz: Vec2<u8>, y: i16) -> Option<&mut Self::UserBlock>;
    fn get_block_at(&self, xz: Vec2<u8>, y: i16) -> Option<&Self::UserBlock>;
    fn set_block_at(
        &mut self,
        block: Self::UserBlock,
        xz: Vec2<u8>,
        y: i16,
    ) -> Result<(), <Self::UserSubchunk as SubChunkTrait>::Err>;

    /// This should return the min and max subchunk index
    fn min_max(&self) -> Vec2<i8>;
}

#[cfg(feature = "default-impl")]
pub mod default_impl {
    use super::*;
    use crate::level::level::LevelError;
    use crate::level::sub_chunk::SubChunkTrait;
    use crate::level::world_block::WorldBlockTrait;
    use std::marker::PhantomData;
    use std::ops::{Deref, DerefMut};
    use std::slice::{Iter, IterMut};
    use std::vec::Vec;

    #[allow(dead_code)]
    pub struct LevelChunk<UserState, UserSubChunkType> {
        bounds: Vec2<i8>,
        xz: Vec2<i32>,
        dim: Dimension,
        sections: Vec<UserSubChunkType>,
        phantom_data: PhantomData<UserState>,
    }

    impl<UserState, UserSubChunkType> Deref for LevelChunk<UserState, UserSubChunkType> {
        type Target = Vec<UserSubChunkType>;
        fn deref(&self) -> &Self::Target {
            &self.sections
        }
    }

    impl<UserState, UserSubChunkType> DerefMut for LevelChunk<UserState, UserSubChunkType> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.sections
        }
    }

    impl<
            UserState,
            UserBlockType: WorldBlockTrait<UserState = UserState>,
            UserSubChunkType: SubChunkTrait<UserState = UserState, BlockType = UserBlockType>,
        > LevelChunk<UserState, UserSubChunkType>
    {
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
        // The base in this context being 0 if the subchunk is positive y and 16 if the subchunk is negative
        fn real_y(raw: i8) -> i8 {
            if raw >= 0 {
                raw
            } else {
                16 + raw
                // This looks a little confusing so ill explain.
                // raw is a negative number meaning 16 + raw is basically 16 - abs(raw)
            }
        }

        pub fn iter(&self) -> Iter<'_, UserSubChunkType> {
            self.sections.iter()
        }

        pub fn iter_mut(&mut self) -> IterMut<'_, UserSubChunkType> {
            self.sections.iter_mut()
        }

        pub fn get_subchunk(&self, idx: i8) -> Option<&UserSubChunkType> {
            self.sections.get(Self::real_y(idx) as usize)
        }

        pub fn get_subchunk_mut(&mut self, idx: i8) -> Option<&mut UserSubChunkType> {
            self.sections.get_mut(Self::real_y(idx) as usize)
        }

        pub fn set_subchunk(&mut self, y: i8, mut chnk: UserSubChunkType) {
            chnk.set_y(y);
            self[Self::real_y(y) as usize] = chnk;
        }

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
            }
        }
    }

    impl<
            UserState,
            UserBlockType: WorldBlockTrait<UserState = UserState>,
            UserSubChunkType: SubChunkTrait<UserState = UserState, BlockType = UserBlockType>,
            UserLevelInterface: LevelModificationProvider<
                UserSubChunkType = UserSubChunkType,
                UserBlockType = UserBlockType,
                UserState = UserState,
            >,
        > LevelChunkTrait<UserLevelInterface> for LevelChunk<UserState, UserSubChunkType>
    {
        type UserLevel = UserLevelInterface;
        type UserBlock = UserLevelInterface::UserBlockType;
        type UserSubchunk = UserLevelInterface::UserSubChunkType;
        type UserState = UserLevelInterface::UserState;
        type Err = LevelError;

        fn load_from_world(
            min_max: Vec2<i8>,
            xz: Vec2<i32>,
            dim: Dimension,
            level: &mut Self::UserLevel,
        ) -> Result<Self, Self::Err> {
            let total_count = min_max.y - min_max.x;
            let mut subchunk_list: Vec<_> = (min_max.x..min_max.y)
                .map(|y| Self::UserSubchunk::empty(y, level.state()))
                .collect();

            for y in min_max.x..min_max.y {
                let subchunk = level.get_sub_chunk(xz, y, dim)?;
                let idx = Self::true_index(min_max, y) as usize;
                subchunk_list[idx] = subchunk;
            }
            Ok(Self {
                bounds: min_max,
                xz,
                dim,
                sections: subchunk_list,
                phantom_data: PhantomData,
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
    }
}
