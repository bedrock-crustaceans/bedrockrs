use bedrockrs_shared::world::dimension::Dimension;
use bedrockrs_world::level::chunk::{FillFilter, LevelChunkTrait};
use bedrockrs_world::level::level::default_impl::*;
use bedrockrs_world::level::level::ChunkSelectionFilter;

#[cfg(feature = "default-impl")]
#[test]
fn world_test(
) -> Result<(), BedrockLevelError<RawInterface, BedrockSubChunkDecoder, BedrockSubChunk>> {
    let wld_path = std::fs::read_to_string("world_path.txt").ok();
    if wld_path.is_none() {
        println!(
            "Skipping Full Test, Add \"world_path.txt\" next to the Cargo.toml with a world path!"
        );
        return Ok(());
    }
    let wld_path = wld_path.unwrap();

    println!("Loading World");

    let mut level = BedrockLevel::open(&wld_path, false, false, BedrockState {})?;

    println!("Collecting Chunks");
    let chunks = level.get_chunk_keys(ChunkSelectionFilter::Dimension(Dimension::Overworld));

    println!("Collected {} Chunks!", chunks.len());

    let blks = [
        BedrockWorldBlock::new("minecraft:iron_block".to_string()),
        BedrockWorldBlock::new("minecraft:diamond_block".to_string()),
    ];
    let len = chunks.len();

    println!("Filling Chunks");
    for (idx, key) in chunks.into_iter().enumerate() {
        let mut chunk = BedrockChunk::empty(
            key,
            (-4, 20).into(),
            Dimension::Overworld,
            &mut BedrockState {},
        );

        for blk in &blks {
            chunk
                .fill_chunk(
                    blk.clone(),
                    FillFilter::Precedence(Box::new(|_, _, _, _| rand::random::<f32>() > 0.5)),
                )
                .expect("Fill failed");
        }

        chunk.write_to_world(&mut level, None, None)?;

        if idx % (len / 10 + 1) == 0 {
            println!("Wrote {idx} out of {len} chunks!");
        }
    }

    level.close();

    Ok(())
}
