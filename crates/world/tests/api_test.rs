use bedrockrs_shared::world::dimension::Dimension;
use bedrockrs_world::level::chunk::default_impl::LevelChunk;
use bedrockrs_world::level::chunk::{FillFilter, LevelChunkTrait};
use bedrockrs_world::level::level::default_impl;
use bedrockrs_world::level::level::default_impl::{
    BedrockChunk, BedrockLevel, BedrockState, BedrockSubChunk, BedrockWorldBlock,
};
use bedrockrs_world::level::sub_chunk::default_impl::SubChunk;
use bedrockrs_world::level::sub_chunk::SubChunkTrait;
use bedrockrs_world::level::world_block::default_impl::WorldBlock;
use optick_attr::*;

#[cfg(feature = "default-impl")]
#[test]
#[optick_attr::capture("parse_50_chunks")]
#[optick_attr::profile]
fn world_test() {
    let wld_path = match std::fs::read_to_string("world_path.txt") {
        Ok(val) => Some(val),
        Err(_) => None,
    };
    if wld_path.is_none() {
        println!(
            "Skipping Full Test, Add \"world_path.txt\" next to the Cargo.toml with a world path!"
        );
        return;
    }

    let mut level = default_impl::BedrockLevel::open(
        &wld_path.unwrap(),
        false,
        false,
        default_impl::BedrockState {},
    )
    .expect("Unexpected Fail");
    let first_count = level.get_chunk_keys(Dimension::Overworld).len();
    let res = level.get_chunk_keys(Dimension::Overworld);
    println!("there are {} chunks! Parsing 1000! ", res.len());
    for x in 0..1000 {
        let mut chunk = level
            .get_chunk::<BedrockChunk>((-4, 20).into(), res[x], Dimension::Overworld)
            .unwrap();
        // chunk
        //     .fill_chunk(
        //         BedrockWorldBlock::new("minecraft:iron_block".into()),
        //         FillFilter::Blanket,
        //     )
        //     .unwrap();
        level.set_chunk(chunk, None, None).unwrap()
    }
}
