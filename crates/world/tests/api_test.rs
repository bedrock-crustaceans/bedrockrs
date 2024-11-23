use bedrockrs_shared::world::dimension::Dimension;
use bedrockrs_world::level::level::default_impl;
use bedrockrs_world::level::level::default_impl::{BedrockChunk, BedrockSubChunk};
use bedrockrs_world::level::sub_chunk::SubChunkTrait;
use bedrockrs_world::level::world_block::default_impl::WorldBlock;
use std::ops::DerefMut;

#[cfg(feature = "default-impl")]
#[test]
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
    let res = level.get_chunk_keys(Dimension::Overworld);
    println!("there are {} chunks! Wiping", res.len());
    for pos in res {
        level
            .set_chunk(
                BedrockChunk::empty(
                    pos,
                    (-4, 20).into(),
                    Dimension::Overworld,
                    &mut default_impl::BedrockState {},
                ),
                None,
                None,
            )
            .unwrap();
    }
}
