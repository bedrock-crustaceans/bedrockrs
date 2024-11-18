use crate::level::level::LevelModificationProvider;
use bedrockrs_core::Vec2;
use bedrockrs_shared::world::dimension::Dimension;

pub trait LevelChunkTrait<UserLevel: LevelModificationProvider>: Sized {
    type UserLevel = UserLevel;
    type UserBlock = UserLevel::UserBlockType;
    type UserSubchunk = UserLevel::UserSubChunkType;
    type UserState = UserLevel::UserState;
    type Err;

    fn load_from_world(
        min_max: Vec2<i8>,
        xz: Vec2<i32>,
        dim: Dimension,
        level: &mut Self::UserLevel,
        state: &mut Self::UserState,
    ) -> Result<Self, Self::Err>;
}

#[cfg(feature = "default-impl")]
pub mod default_impl {
    use super::*;
    use std::marker::PhantomData;

    #[allow(dead_code)]
    pub struct LevelChunk<UserState, UserSubChunkType> {
        bounds: Vec2<i8>,
        xz: Vec2<i32>,
        dim: Dimension,
        sections: Vec<UserSubChunkType>,
        phantom_data: PhantomData<UserState>,
    }
    impl<
            UserState,
            UserBlockType: WorldBlockTrait<UserState = UserState>,
            UserSubChunkType: SubChunkTrait<UserState = UserState, BlockType = UserBlockType>,
        > LevelChunk<UserState, UserSubChunkType>
    {
        fn true_index(min_max: Vec2<i8>, raw_index: i8) -> i8 {
            #[cfg(debug_assertions)]
            if raw_index < min_max.x {
                panic!("Y out of bounds");
            } else if raw_index > min_max.y {
                panic!("Y out of bounds");
            }
            raw_index + min_max.x.abs()
        }
    }

    impl<
            UserState,
            UserBlockType: WorldBlockTrait<UserState = UserState>,
            UserSubChunkType: SubChunkTrait<UserState = UserState, BlockType = UserBlockType>,
            UserLevelInterface: LevelModificationProvider<UserSubChunkType = UserSubChunkType>,
        > LevelChunkTrait<UserLevelInterface> for LevelChunk<UserState, UserSubChunkType>
    {
        type Err = LevelError;

        fn load_from_world(
            min_max: Vec2<i8>,
            xz: Vec2<i32>,
            dim: Dimension,
            level: &mut Self::UserLevel,
            _: &mut Self::UserState,
        ) -> Result<Self, Self::Err> {
            let total_count = min_max.y - min_max.x;
            let mut subchunk_list = Vec::<Self::UserSubchunk>::with_capacity(total_count as usize);
            for y in min_max.x..=min_max.y {
                let subchunk = level.get_sub_chunk(xz, y, dim)?;
                subchunk_list[Self::true_index(min_max, y) as usize] = subchunk;
            }
            Ok(Self {
                bounds: min_max,
                xz,
                dim,
                sections: subchunk_list,
                phantom_data: PhantomData,
            })
        }
    }
}
