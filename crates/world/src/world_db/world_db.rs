use std::fmt::{Debug, Formatter};
use std::{collections::HashMap, path::PathBuf};
use std::path::Path;
use bedrockrs_nbt::{endian::little_endian::NbtLittleEndian, NbtTag};
use mojang_leveldb::{error::DBError, Options, ReadOptions, WriteBatch, WriteOptions, DB};
use uuid::Uuid;

use bedrockrs_shared::world::dimension::Dimension;

use crate::error::WorldError;

use super::subchunk::SubChunk;
use super::{create_key, RecordType};

pub struct WorldDB {
    db: DB,
}

const READ_OPTIONS: ReadOptions = ReadOptions {
    fill_cache: true,
    verify_checksums: true,
};

const WRITE_OPTIONS: WriteOptions = WriteOptions { sync: true };

impl WorldDB {
    /// Opens a world from a directory.
    ///
    /// The leveldb database is in the `db` subdirectory.
    pub fn open(directory: &Path) -> Result<Self, DBError> {
        Ok(WorldDB {
            db: DB::open(
                &directory.join("db").display().to_string(),
                Options {
                    compression: mojang_leveldb::CompressionType::ZlibRaw,
                    create_if_missing: true,
                },
            )?,
        })
    }

    /// Read a player's NBT data for this world
    pub fn get_player(&self, uuid: Uuid) -> Result<Option<HashMap<String, NbtTag>>, WorldError> {
        let mut str = uuid.to_string();
        str.insert_str(0, "player_");

        match self.db.get(READ_OPTIONS, str.as_bytes()) {
            Ok(maybe_bytes) => match maybe_bytes {
                Some(bytes) => {
                    let u8_bytes = bytes.get().into();
                    match NbtTag::nbt_deserialize_vec::<NbtLittleEndian>(&u8_bytes) {
                        Ok((_, tag)) => match tag {
                            NbtTag::Compound(ctag) => Ok(Some(ctag)),
                            _ => Err(WorldError::FormatError(
                                "Player data tag is not a compound tag".to_string(),
                            )),
                        },
                        Err(e) => Err(WorldError::NbtError(e)),
                    }
                }
                None => Ok(None),
            },
            Err(e) => Err(WorldError::DBError(e)),
        }
    }

    /// Set a player's NBT data for this world
    pub fn set_player(
        &mut self,
        uuid: Uuid,
        data: HashMap<String, NbtTag>,
    ) -> Result<(), WorldError> {
        let tag = NbtTag::Compound(data);
        match tag.nbt_serialize_vec::<NbtLittleEndian>("") {
            Ok(sertag) => {
                let mut str = uuid.to_string();
                str.insert_str(0, "player_");

                let mut wb = WriteBatch::new();

                wb.put(str.as_bytes(), &sertag);

                match self.db.write(WRITE_OPTIONS, wb) {
                    Ok(()) => Ok(()),
                    Err(dberr) => Err(WorldError::DBError(dberr)),
                }
            }
            Err(e) => Err(WorldError::NbtError(e)),
        }
    }

    pub fn get_subchunk(
        &self,
        x: i32,
        y: u8,
        z: i32,
        dimension: Dimension,
    ) -> Result<Option<SubChunk>, DBError> {
        let bytes = self.db.get(
            READ_OPTIONS,
            create_key(x, z, dimension, RecordType::SubChunkPrefix { y }).as_slice(),
        )?;
        
        Ok(bytes.map(|x| SubChunk::load(&(x.get()))))
    }

    pub fn set_subchunk(
        &mut self,
        x: i32,
        y: u8,
        z: i32,
        dimension: Dimension,
        subchunk: SubChunk,
    ) -> Result<(), DBError> {
        let mut wb = WriteBatch::new();
        wb.put(
            &create_key(x, z, dimension, RecordType::SubChunkPrefix { y }),
            &subchunk.save(),
        );
        self.db.write(WRITE_OPTIONS, wb)?;

        Ok(())
    }
}

impl Debug for WorldDB {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // can't be printed
        write!(f, "WorldDB(...)")
    }
}
