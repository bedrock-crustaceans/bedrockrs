use bedrockrs_shared::world::dimension::Dimension;
use bedrockrs_world::level::level::default_impl;
use bedrockrs_world::level::level::default_impl::State;
use bedrockrs_world::level::sub_chunk::default_impl::SubChunk;
use bedrockrs_world::level::sub_chunk::SubChunkTrait;
use bedrockrs_world::level::world_block::default_impl::WorldBlock;

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

    println!("{}", std::mem::size_of::<WorldBlock<default_impl::State>>());

    let mut level =
        default_impl::BedrockLevel::open(&wld_path.unwrap(), false, true, default_impl::State {})
            .expect("Unexpected Fail");
    let mut chunk = level
        .get_sub_chunk((0, 0).into(), 8, Dimension::Overworld)
        .expect("Fail");

    println!(
        "{:?}",
        chunk.set_block((5, 1, 5).into(), WorldBlock::new("minecraft:stone".into()))
    );
    const range: i32 = 20;
    let chnk = SubChunk::<WorldBlock<State>, State>::empty(1, &mut State {});
    for x in -range..=range {
        for z in -range..=range {
            for y in -2..=10i8 {
                level
                    .set_sub_chunk(chnk.clone(), (x, z).into(), y, Dimension::Overworld)
                    .unwrap()
            }
        }
    }
}
