use crate::level::biome::BiomeTransitionInformation;
use crate::types::binary::BinaryBuffer;
use bedrockrs_core::Vec3;

pub type BlockLayer<T> = (Box<[u16; 4096]>, Vec<T>);

#[allow(dead_code)]
pub struct SubchunkTransitionalData<BlockType> {
    y_level: i8,
    data_version: u8,
    layers: Vec<BlockLayer<BlockType>>,
}

impl<BlockType> SubchunkTransitionalData<BlockType> {
    pub fn new(y_level: i8, layer_count: usize) -> Self {
        Self {
            y_level,
            data_version: 9,
            layers: Vec::with_capacity(layer_count),
        }
    }

    pub fn new_layer(&mut self, data: (Box<[u16; 4096]>, Vec<BlockType>)) {
        self.layers.push(data);
    }
}

pub trait SubChunkDecoder {
    type Err;
    type BlockType;
    type UserState;

    fn decode_bytes_as_chunk(
        bytes: &mut BinaryBuffer,
        state: &mut Self::UserState,
    ) -> Result<SubchunkTransitionalData<Self::BlockType>, Self::Err>;
    fn write_as_bytes(
        chunk_state: SubchunkTransitionalData<Self::BlockType>,
        network: bool,
        state: &mut Self::UserState,
    ) -> Result<Vec<u8>, Self::Err>;
}

pub trait SubChunkTrait: Sized {
    type Err;
    type BlockType;
    type UserState;

    fn empty(y_index: i8, state: &mut Self::UserState) -> Self;
    fn decode_from_raw(
        data: SubchunkTransitionalData<Self::BlockType>,
        state: &mut Self::UserState,
    ) -> Result<Self, Self::Err>;
    fn to_raw(
        &self,
        y_level: i8,
        state: &mut Self::UserState,
    ) -> Result<SubchunkTransitionalData<Self::BlockType>, Self::Err>;
    fn get_block(&self, xyz: Vec3<u8>) -> Option<&Self::BlockType>;
    fn get_block_mut(&mut self, xyz: Vec3<u8>) -> Option<&mut Self::BlockType>;
    fn set_block(&mut self, xyz: Vec3<u8>, block: Self::BlockType) -> Result<(), Self::Err>;

    fn get_active_layer(&self) -> u8;
    fn set_active_layer(&mut self, idx: u8);

    fn add_sub_layer(&mut self, state: &mut Self::UserState);
    fn get_sub_layer_count(&self) -> usize;

    fn get_y(&self) -> i8;
    fn set_y(&mut self, y: i8) -> i8;

    fn state_clone(&self, _: &mut Self::UserState) -> Self;

    fn is_empty(&self) -> bool;
}

#[cfg(feature = "default-impl")]
pub mod default_impl {
    use super::*;
    use crate::level::world_block::{BlockTransitionalState, WorldBlockTrait};
    use crate::types::binary::BinaryInterfaceError;
    use crate::types::miner::idx_3_to_1;
    use anyhow::anyhow;
    use byteorder::LittleEndian;
    use std::io::Cursor;
    use std::marker::PhantomData;

    pub struct SubChunkDecoderImpl<UserBlockType: WorldBlockTrait, UserState> {
        _block_marker: PhantomData<UserBlockType>,
        _state_marker: PhantomData<UserState>,
    }

    impl<UserBlockType: WorldBlockTrait<UserState = UserState>, UserState> SubChunkDecoder
        for SubChunkDecoderImpl<UserBlockType, UserState>
    {
        type Err = anyhow::Error;
        type BlockType = UserBlockType;
        type UserState = UserState;

        fn decode_bytes_as_chunk(
            bytes: &mut BinaryBuffer,
            state: &mut Self::UserState,
        ) -> Result<SubchunkTransitionalData<Self::BlockType>, Self::Err> {
            let version = bytes
                .read::<LittleEndian, u8>()
                .ok_or(anyhow!("Failed To Read Subchunk Version"))?;
            if version != 8 && version != 9 {
                return Err(anyhow!("Unknown Subchunk Version"));
            }

            let storage_layer_count = bytes
                .read::<LittleEndian, u8>()
                .ok_or(anyhow!("Failed To Read Layer Count"))?;
            let y_index = bytes
                .read::<LittleEndian, i8>()
                .ok_or(anyhow!("Failed To Read Y Index"))?;

            let mut transitiondata =
                SubchunkTransitionalData::new(y_index, storage_layer_count as usize);

            for _ in 0..storage_layer_count {
                let palette_type = bytes
                    .read::<LittleEndian, u8>()
                    .ok_or(anyhow!("Failed To Read Palette Type"))?;
                if palette_type & 0x1 == 1 {
                    // TODO: Network Support
                    panic!("Network Not Supported!");
                }
                let bits_per_block = palette_type >> 1;
                let blocks_per_word = 32 / bits_per_block;
                let word_count = (4096 + (blocks_per_word as i32) - 1) / (blocks_per_word as i32);
                let mask = (1 << bits_per_block) - 1;
                let mut pos = 0usize;
                let mut block_indices = Box::new([0u16; 4096]);

                for _ in 0..word_count {
                    let mut word = bytes
                        .read::<LittleEndian, u32>()
                        .ok_or(anyhow!("Failed To Read Word"))?;
                    for _ in 0..blocks_per_word {
                        let index: u16 = (word & mask) as u16;
                        if pos == 4096 {
                            break;
                        }
                        block_indices[pos] = index;
                        word >>= bits_per_block;
                        pos += 1;
                    }
                }

                // TODO: Handle Network endian.
                let palette_count = bytes
                    .read::<LittleEndian, u32>()
                    .ok_or(anyhow!("Failed To Read Palette Count"))?;
                let mut blocks = Vec::with_capacity(palette_count as usize);
                for _ in 0_usize..palette_count as usize {
                    let cursor =
                        &mut Cursor::new(bytes.poll_buffer().ok_or(anyhow!("Failed To Slice"))?);

                    blocks.push(Self::BlockType::from_transition(
                        nbtx::from_bytes::<nbtx::LittleEndian, BlockTransitionalState>(cursor)?,
                        state,
                    ));
                    let pos = cursor.position() as isize;
                    bytes.rebase(pos);
                }
                transitiondata.new_layer((block_indices, blocks));
            }
            Ok(transitiondata)
        }

        // TODO: Handle 0, 2, 3, 4 ,5 ,6 7, also handle 1
        fn write_as_bytes(
            chunk_state: SubchunkTransitionalData<Self::BlockType>,
            network: bool,
            state: &mut Self::UserState,
        ) -> Result<Vec<u8>, Self::Err> {
            if network {
                panic!("Network handling isn't implemented yet");
            }
            let mut buffer = BinaryBuffer::new();
            buffer
                .write::<LittleEndian, u8>(chunk_state.data_version)?
                .write::<LittleEndian, u8>(chunk_state.layers.len() as u8)?
                .write::<LittleEndian, i8>(chunk_state.y_level)?;
            for layer in chunk_state.layers {
                let bits_per_block = bits_needed_to_store(layer.1.len() as u32);
                buffer.write::<LittleEndian, u8>(bits_per_block << (1 + (network as u8)))?;

                let mut current_word = 0u32;
                let mut bits_written = 0;
                layer.0.iter().try_for_each(|element| {
                    let element = *element as u32;
                    if bits_written + bits_per_block > 32 {
                        buffer.write::<LittleEndian, u32>(current_word)?;
                        current_word = 0;
                        bits_written = 0;
                    }

                    current_word = current_word + (element << bits_written);
                    bits_written += bits_per_block;
                    Ok::<(), BinaryInterfaceError>(())
                })?;
                if bits_written != 0 {
                    buffer.write::<LittleEndian, u32>(current_word)?;
                }
                buffer.write::<LittleEndian, u32>(layer.1.len() as u32)?;
                for blk in layer.1 {
                    buffer.write::<LittleEndian, &[u8]>(&nbtx::to_le_bytes(
                        &blk.into_transition(state),
                    )?)?;
                }
            }
            Ok(buffer.into())
        }
    }

    pub struct SubChunk<UserBlockType: WorldBlockTrait, UserState> {
        blocks: Vec<Box<[UserBlockType; 4096]>>,
        y_index: i8,
        active_layer: u8,
        is_empty: bool,
        _state_tag: PhantomData<UserState>,
    }
    impl<UserBlockType: WorldBlockTrait<UserState = UserState>, UserState>
        SubChunk<UserBlockType, UserState>
    {
        pub fn force_non_empty(&mut self) {
            self.is_empty = false;
        }

        pub fn force_empty(&mut self) {
            self.is_empty = false;
        }

        pub fn replace(&mut self, other: Self) {
            self.blocks = other.blocks;
            self.y_index = other.y_index;
            self.active_layer = other.active_layer;
            self.is_empty = other.is_empty;
        }
    }

    impl<UserBlockType: WorldBlockTrait<UserState = UserState>, UserState> SubChunkTrait
        for SubChunk<UserBlockType, UserState>
    {
        type Err = anyhow::Error;
        type BlockType = UserBlockType;
        type UserState = UserState;

        fn empty(y_index: i8, state: &mut Self::UserState) -> Self {
            let mut val = Self {
                blocks: Vec::with_capacity(1),
                y_index,
                active_layer: 0,
                is_empty: true,
                _state_tag: PhantomData,
            };
            val.blocks.push(Box::new(std::array::from_fn(|_| {
                Self::BlockType::air(state)
            })));
            val
        }

        fn decode_from_raw(
            data: SubchunkTransitionalData<Self::BlockType>,
            state: &mut Self::UserState,
        ) -> Result<Self, Self::Err> {
            let mut building = Self::empty(data.y_level, state);
            building.is_empty = false;
            for _ in 1..data.layers.len() {
                building.add_sub_layer(state);
            }
            for (layer_index, (indices, blocks)) in data.layers.into_iter().enumerate() {
                building.set_active_layer(layer_index as u8);

                for z in 0u8..16u8 {
                    for y in 0u8..16u8 {
                        for x in 0u8..16u8 {
                            building.set_block(
                                (x, y, z).into(),
                                Self::BlockType::from_other(
                                    &blocks[indices[idx_3_to_1::<u8>((x, y, z).into(), 16, 16)]
                                        as usize],
                                ),
                            )?;
                        }
                    }
                }
            }
            Ok(building)
        }

        fn to_raw(
            &self,
            y_level: i8,
            _: &mut Self::UserState,
        ) -> Result<SubchunkTransitionalData<Self::BlockType>, Self::Err> {
            let mut layers: Vec<BlockLayer<Self::BlockType>> =
                Vec::with_capacity(self.blocks.len());
            for layer in 0..self.blocks.len() {
                layers.push(self.encode_single_layer(layer));
            }
            Ok(SubchunkTransitionalData {
                layers,
                data_version: 9, // TODO: Change this to be configurable
                y_level,
            })
        }

        fn get_block(&self, xyz: Vec3<u8>) -> Option<&Self::BlockType> {
            let layer: &[Self::BlockType; 4096] = self.blocks.get(self.active_layer as usize)?;
            layer.get(idx_3_to_1::<u8>(xyz, 16u8, 16u8))
        }

        fn get_block_mut(&mut self, xyz: Vec3<u8>) -> Option<&mut Self::BlockType> {
            let layer: &mut [Self::BlockType; 4096] =
                self.blocks.get_mut(self.active_layer as usize)?;
            layer.get_mut(idx_3_to_1::<u8>(xyz, 16u8, 16u8))
        }

        fn set_block(&mut self, xyz: Vec3<u8>, block: Self::BlockType) -> Result<(), Self::Err> {
            let layer: &mut [Self::BlockType; 4096] = self
                .blocks
                .get_mut(self.active_layer as usize)
                .ok_or(anyhow!("Failed to get layer"))?;
            layer[idx_3_to_1::<u8>(xyz, 16u8, 16u8)] = block;
            self.is_empty = false;
            Ok(())
        }
        fn get_active_layer(&self) -> u8 {
            self.active_layer
        }

        fn set_active_layer(&mut self, idx: u8) {
            if idx as usize >= self.blocks.len() {
                panic!(
                    "Selected subchunk index outside of valid range!, Layer Count: {}",
                    self.blocks.len()
                )
            }
            self.active_layer = idx;
        }

        fn add_sub_layer(&mut self, state: &mut Self::UserState) {
            self.blocks.push(Box::new(std::array::from_fn(|_| {
                Self::BlockType::air(state)
            })));
        }

        fn get_sub_layer_count(&self) -> usize {
            self.blocks.len()
        }

        fn get_y(&self) -> i8 {
            self.y_index
        }

        fn set_y(&mut self, y: i8) -> i8 {
            self.y_index = y;
            self.y_index
        }

        fn state_clone(&self, _: &mut Self::UserState) -> Self {
            Self {
                blocks: self.blocks.clone(),
                y_index: self.y_index,
                active_layer: self.active_layer,
                is_empty: self.is_empty,
                _state_tag: PhantomData,
            }
        }

        fn is_empty(&self) -> bool {
            self.is_empty
        }
    }

    impl<UserBlockType: WorldBlockTrait<UserState = UserState>, UserState>
        SubChunk<UserBlockType, UserState>
    {
        fn encode_single_layer(&self, layer_override: usize) -> BlockLayer<UserBlockType> {
            let mut indices = Box::new([0u16; 4096]);
            let mut unique_block_array = Vec::new();
            let layer = &self.blocks[layer_override];
            for z in 0..16u8 {
                for y in 0..16u8 {
                    for x in 0..16u8 {
                        let current_block = &layer[idx_3_to_1((x, y, z).into(), 16, 16)];
                        if let Some(index) = unique_block_array
                            .iter()
                            .position(|ele| ele == current_block)
                        {
                            indices[idx_3_to_1((x, y, z).into(), 16, 16)] = index as u16;
                        } else {
                            unique_block_array.push(current_block.clone());
                            indices[idx_3_to_1((x, y, z).into(), 16, 16)] =
                                (unique_block_array.len() - 1) as u16;
                        }
                    }
                }
            }
            (indices, unique_block_array)
        }
    }

    impl<UserBlockType: WorldBlockTrait, UserState> Clone for SubChunk<UserBlockType, UserState> {
        fn clone(&self) -> Self {
            Self {
                active_layer: self.active_layer,
                _state_tag: PhantomData,
                blocks: self.blocks.clone(),
                is_empty: self.is_empty,
                y_index: self.y_index.clone(),
            }
        }
    }
}

fn bits_needed_to_store(val: u32) -> u8 {
    if val == 0 {
        1
    } else {
        (32 - val.leading_zeros()) as u8
    }
}
