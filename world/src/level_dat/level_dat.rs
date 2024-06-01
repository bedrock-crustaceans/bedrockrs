use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use bedrock_core::difficulty::Difficulty;
use bedrock_core::dimension::Dimension;
use bedrock_core::stream::read::ByteStreamRead;
use nbt::endian::little_endian::NbtLittleEndian;
use nbt::NbtTag;

use crate::error::WorldError;
use crate::level_dat::abilities::WorldAbilities;

pub struct WorldGameRules {
    allow_destructive_objects: NbtTag,
}

#[derive(Debug)]
pub struct LevelDat {
    /// Specifies the name of the world. (NBT entry: `LevelName`)
    pub level_name: String,

    /// Version of Bedrock Edition Storage Tool, currently is 10. (NBT entry: `StorageVersion`)
    pub format_version: i32,

    /// The default permissions for players in the world. (NBT entry: `abilities`)
    pub abilities: WorldAbilities,

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

impl LevelDat {
    pub fn open(directory: &PathBuf) -> Result<(i32, i32, Self), WorldError> {
        let mut file = match File::open(directory.join("level.dat")) {
            Ok(v) => v,
            Err(e) => {
                return Err(WorldError::FormatError(e.to_string()));
            }
        };

        let mut data = vec![];

        match file.read_to_end(&mut data) {
            Ok(_) => {}
            Err(e) => {
                return Err(WorldError::FormatError(e.to_string()));
            }
        };

        let mut stream = ByteStreamRead::from(data);

        let version = match stream.read_i32le() {
            Ok(v) => v.0,
            Err(e) => {
                return Err(WorldError::FormatError(e.to_string()));
            }
        };

        let length = match stream.read_i32le() {
            Ok(v) => v.0,
            Err(e) => {
                return Err(WorldError::FormatError(e.to_string()));
            }
        };

        let (_, nbt) = match NbtTag::nbt_deserialize::<NbtLittleEndian>(&mut stream) {
            Ok(v) => v,
            Err(e) => {
                return Err(WorldError::NbtError(e));
            }
        };

        let level_dat = match Self::parse(nbt) {
            Ok(v) => v,
            Err(e) => {
                return Err(e);
            }
        };

        Ok((version, length, level_dat))
    }

    pub fn parse(tag: NbtTag) -> Result<Self, WorldError> {
        fn get_string(map: &mut HashMap<String, NbtTag>, key: &str) -> Result<String, WorldError> {
            match map.remove(key) {
                Some(NbtTag::String(v)) => Ok(v),
                Some(other) => Err(WorldError::FormatError(format!(
                    "Expected `{}` in LevelDat to be of type String, got {:?}",
                    key, other
                ))),
                None => Err(WorldError::FormatError(format!(
                    "Missing field `{}` in LevelDat",
                    key
                ))),
            }
        }

        fn get_byte_as_bool(
            map: &mut HashMap<String, NbtTag>,
            key: &str,
        ) -> Result<bool, WorldError> {
            match map.remove(key) {
                Some(NbtTag::Byte(v)) => Ok(v != 0),
                Some(other) => Err(WorldError::FormatError(format!(
                    "Expected `{}` in LevelDat to be of type Byte, got {:?}",
                    key, other
                ))),
                None => Err(WorldError::FormatError(format!(
                    "Missing field `{}` in LevelDat",
                    key
                ))),
            }
        }

        fn get_int32(map: &mut HashMap<String, NbtTag>, key: &str) -> Result<i32, WorldError> {
            match map.remove(key) {
                Some(NbtTag::Int32(v)) => Ok(v),
                Some(other) => Err(WorldError::FormatError(format!(
                    "Expected `{}` in LevelDat to be of type Int32, got {:?}",
                    key, other
                ))),
                None => Err(WorldError::FormatError(format!(
                    "Missing field `{}` in LevelDat",
                    key
                ))),
            }
        }

        fn get_int32_option(
            map: &mut HashMap<String, NbtTag>,
            key: &str,
        ) -> Result<Option<i32>, WorldError> {
            match map.remove(key) {
                Some(NbtTag::Int32(v)) => Ok(Some(v)),
                Some(other) => Err(WorldError::FormatError(format!(
                    "Expected `{}` in LevelDat to be of type Int32, got {:?}",
                    key, other
                ))),
                None => Ok(None),
            }
        }

        fn get_int64(map: &mut HashMap<String, NbtTag>, key: &str) -> Result<i64, WorldError> {
            match map.remove(key) {
                Some(NbtTag::Int64(v)) => Ok(v),
                Some(other) => Err(WorldError::FormatError(format!(
                    "Expected `{}` in LevelDat to be of type Int64, got {:?}",
                    key, other
                ))),
                None => Err(WorldError::FormatError(format!(
                    "Missing field `{}` in LevelDat",
                    key
                ))),
            }
        }

        fn get_compound(
            map: &mut HashMap<String, NbtTag>,
            key: &str,
        ) -> Result<HashMap<String, NbtTag>, WorldError> {
            match map.remove(key) {
                Some(NbtTag::Compound(v)) => Ok(v),
                Some(other) => Err(WorldError::FormatError(format!(
                    "Expected `{}` in LevelDat to be of type Compound, got {:?}",
                    key, other
                ))),
                None => Err(WorldError::FormatError(format!(
                    "Missing field `{}` in LevelDat",
                    key
                ))),
            }
        }

        match tag {
            NbtTag::Compound(mut map) => Ok(Self {
                level_name: get_string(&mut map, "LevelName")?,
                format_version: get_int32(&mut map, "StorageVersion")?,
                abilities: WorldAbilities::parse(match map.remove("abilities") {
                    Some(v) => v,
                    None => Err(WorldError::FormatError(format!(
                        "Missing field `abilities` in LevelDat"
                    )))?,
                })?,
                experiments: {
                    let mut nbt = get_compound(&mut map, "experiments")?;
                    let mut experiments = HashMap::new();

                    for (name, tag) in nbt.iter() {
                        experiments.insert(name.clone(), match tag {
                            NbtTag::Byte(v) => { *v != 0 }
                            other => { Err(WorldError::FormatError(format!("Expected `{}` in LevelDat experiments to be of type Byte, got {:?}", name, other)))? }
                        });
                    }

                    experiments
                },
                difficulty: match get_int32(&mut map, "Difficulty")? {
                    0 => Difficulty::Peaceful,
                    1 => Difficulty::Easy,
                    2 => Difficulty::Normal,
                    3 => Difficulty::Hard,
                    other => Err(WorldError::FormatError(format!(
                        "Value for `Difficulty` is out of bounds, got {:?}",
                        other
                    )))?,
                },
                dimension: match get_int32_option(&mut map, "Dimension")? {
                    Some(1) => Some(Dimension::Overworld),
                    Some(2) => Some(Dimension::Nether),
                    Some(3) => Some(Dimension::End),
                    Some(other) => Err(WorldError::FormatError(format!(
                        "Value for `Dimension` is out of bounds, got {:?}",
                        other
                    )))?,
                    None => None,
                },
                bonus_chest_enabled: get_byte_as_bool(&mut map, "bonusChestEnabled")?,
                bonus_chest_spawned: get_byte_as_bool(&mut map, "bonusChestSpawned")?,
                seed: get_int64(&mut map, "RandomSeed")?,
                current_tick: get_int64(&mut map, "currentTick")?,
                spawn_x: get_int32(&mut map, "SpawnX")?,
                spawn_y: get_int32(&mut map, "SpawnY")?,
                spawn_z: get_int32(&mut map, "SpawnZ")?,
                cheats: get_byte_as_bool(&mut map, "commandsEnabled")?,
            }),
            other => Err(WorldError::FormatError(format!(
                "Expected root tag in LevelDat to be of type Compound, got {:?}",
                other
            ))),
        }
    }
}
