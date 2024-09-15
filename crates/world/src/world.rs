use std::path::{Path, PathBuf};

use crate::error::WorldError;
use crate::level_dat::LevelDat;
use crate::world_db::WorldDB;

/// Represents a Minecraft Bedrock world.
#[derive(Debug)]
pub struct World {
    pub db: WorldDB,
    pub level_dat: LevelDat,
    pub format_version: i32,
}

impl World {
    /// Opens an unzipped Minecraft Bedrock Edition world and returns a [`World`] object with
    /// the data about the world contained in it.
    pub fn open(directory: impl AsRef<Path>) -> Result<World, WorldError> {
        // Convert the given path into a PathBuf
        let directory: PathBuf = directory.as_ref().to_path_buf();

        // Read the world settings/specifications from the level.dat file.
        // (in world/level.dat and world/level.dat_old)
        todo!("Help");
        // let (version, _, level_dat) = match LevelDat::open(&directory) {
        //     Ok(v) => v,
        //     Err(e) => {
        //         return Err(e);
        //     }
        // };

        // Read the world name from the levelname.txt file
        // (in world/levelname.txt)
        // Its stored twice:
        // - in the levelname.txt file
        // - as a field in the level.dat file (world/level.dat and world/level.dat_old)

        // Ok(World {
        //     // Read the LevelDB database (in world/db/*)
        //     db: match WorldDB::open(&PathBuf::from(&directory)) {
        //         Ok(v) => v,
        //         Err(e) => {
        //             return Err(WorldError::DBError(e));
        //         }
        //     },
        //     level_dat,
        //     format_version: 2,
        // })
    }
}
