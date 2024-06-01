use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::error::WorldError;
use crate::level_dat::LevelDat;
use crate::world_db::WorldDB;

#[derive(Debug)]
pub struct World {
    pub db: WorldDB,
    pub level_dat: LevelDat,
    pub format_version: i32,
    pub name: String,
}

impl World {
    pub fn open(directory: impl AsRef<Path>) -> Result<World, WorldError> {
        let directory: PathBuf = directory.as_ref().to_path_buf();

        let (version, _, level_dat) = match LevelDat::open(&directory) {
            Ok(v) => v,
            Err(e) => {
                return Err(e);
            }
        };

        let name = match File::open(&directory.join("levelname.txt")) {
            Ok(mut v) => {
                let mut string = String::new();
                match v.read_to_string(&mut string) {
                    Ok(_) => {}
                    Err(e) => Err(WorldError::FormatError(format!(
                        "Error while reading \"levelname.txt\": {:?}",
                        e.to_string()
                    )))?,
                };

                string
            }
            Err(e) => Err(WorldError::FormatError(format!(
                "Error while reading \"levelname.txt\": {:?}",
                e.to_string()
            )))?,
        };

        Ok(World {
            db: match WorldDB::open(&PathBuf::from(&directory)) {
                Ok(v) => v,
                Err(e) => {
                    return Err(WorldError::DBError(e));
                }
            },
            level_dat,
            format_version: version,
            name,
        })
    }
}
