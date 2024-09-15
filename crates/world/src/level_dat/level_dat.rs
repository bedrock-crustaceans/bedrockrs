use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};

use byteorder::{LittleEndian, ReadBytesExt};

use bedrockrs_shared::world::difficulty::Difficulty;
use bedrockrs_shared::world::dimension::Dimension;

use crate::error::WorldError;
use crate::level_dat::abilities::LevelDatAbilities;

/// A struct representing the data found in the `level.dat` and `level.dat_old` files for
/// the Minecraft Bedrock Level Format.
///
/// The `level.dat` is still in uncompressed NBT format.
/// The file begins with an 8-byte header, consisting of a little-endian 4-byte integer
/// indicating the version of the file, which is currently 10. It is followed by another integer
/// containing the length of the file, minus the header.
#[derive(Debug)]
pub struct LevelDat {
    /// Specifies the name of the world. (NBT entry: `LevelName`)
    pub level_name: String,

    /// Version of Bedrock Edition Storage Tool, currently is 10. (NBT entry: `StorageVersion`)
    pub format_version: i32,

    /// The default permissions for players in the world. (NBT entry: `abilities`)
    pub abilities: LevelDatAbilities,

    /// A key value pair map for each experiment that Minecraft has/had.
    /// It is impossible to parse all Experiments due to frequent changes.
    /// (NBT entry: `experiments`)
    pub experiments: HashMap<String, bool>,

    /// The current difficulty setting. (NBT entry: `Difficulty`)
    pub difficulty: Difficulty,
    /// The dimension the player is in. (NBT entry: `Dimension`)
    pub dimension: Option<Dimension>,

    /// If the bonus chest is enabled. (NBT entry: `bonusChestEnabled`)
    pub bonus_chest_enabled: bool,
    /// If the bonus chest has been placed in the world.
    /// Turning this to false spawns another bonus chest near the spawn coordinates.
    /// (NBT entry: `bonusChestSpawned`)
    pub bonus_chest_spawned: bool,

    /// Specifies the seed of the level. (NBT entry: `RandomSeed`)
    pub seed: i64,

    /// Specifies the current tick of the level. Where 20 ticks are equal to 1 second.
    /// (NBT entry: `currentTick`)
    pub current_tick: i64,

    /// The X coordinate of the player's spawn position. Defaults to 0.
    /// (NBT entry: `SpawnX`)
    pub spawn_x: i32,
    /// The Y coordinate of the player's spawn position. Defaults to 64.
    /// (NBT entry: `SpawnY`)
    pub spawn_y: i32,
    /// The Z coordinate of the player's spawn position. Defaults to 0.
    /// (NBT entry: `SpawnZ`)
    pub spawn_z: i32,

    /// If cheats are on. (NBT entry: `commandsEnabled`)
    pub cheats: bool,
}
