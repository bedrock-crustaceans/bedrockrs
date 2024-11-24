use bedrockrs_shared::world::dimension::Dimension;
use bedrockrs_world::level::level::default_impl;
use bedrockrs_world::level::level::default_impl::{BedrockChunk, BedrockSubChunk};
use bedrockrs_world::level::sub_chunk::SubChunkTrait;
use bedrockrs_world::level::world_block::default_impl::WorldBlock;
use optick_attr::*;
use std::ops::DerefMut;

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
        true,
        default_impl::BedrockState {},
    )
    .expect("Unexpected Fail");
    println!("First: {:?}", level.get_chunk_keys(Dimension::Overworld));
    let first_count = level.get_chunk_keys(Dimension::Overworld).len();
    // assert_eq!(level.get_chunk_keys(Dimension::Overworld));
    // let res = level.get_chunk_keys(Dimension::Overworld);
    // println!("there are {} chunks! Parsing All! ", res.len());
    // for x in 0..res.len() {
    //     let _ = level.get_chunk::<BedrockChunk>((-4, 20).into(), res[x], Dimension::Overworld);
    // }
    drop(level);

    let wld_path = match std::fs::read_to_string("world_path.txt") {
        Ok(val) => Some(val),
        Err(_) => None,
    };
    let mut level = default_impl::BedrockLevel::open(
        &wld_path.unwrap(),
        false,
        true,
        default_impl::BedrockState {},
    )
    .expect("Unexpected Fail");
    println!("Second: {:?}", level.get_chunk_keys(Dimension::Overworld));
    let second_count = level.get_chunk_keys(Dimension::Overworld).len();
    //assert_eq!(first_count, second_count)
}
