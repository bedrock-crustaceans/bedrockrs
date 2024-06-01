use std::fmt::{Debug, Formatter};
use std::{collections::HashMap, path::PathBuf};

use bedrock_core::uuid::UUID;
use mojang_leveldb::{error::DBError, Options, ReadOptions, WriteBatch, WriteOptions, DB};
use nbt::{endian::little_endian::NbtLittleEndian, NbtTag};

use crate::error::WorldError;

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
    pub fn open(directory: &PathBuf) -> Result<Self, DBError> {
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
    pub fn get_player(&self, uuid: UUID) -> Result<Option<HashMap<String, NbtTag>>, WorldError> {
        let mut str = uuid.to_string();
        str.insert_str(0, "player_");

        match self
            .db
            .get(READ_OPTIONS, str_to_ascii_i8(&str).unwrap().as_slice())
        {
            Ok(maybe_bytes) => match maybe_bytes {
                Some(bytes) => {
                    let u8_bytes = vec_i8_into_u8(bytes.get().into());
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
        uuid: UUID,
        data: HashMap<String, NbtTag>,
    ) -> Result<(), WorldError> {
        let tag = NbtTag::Compound(data);
        match tag.nbt_serialize_vec::<NbtLittleEndian>("") {
            Ok(sertag) => {
                let byte_nbt = vec_u8_into_i8(sertag);

                let mut str = uuid.to_string();
                str.insert_str(0, "player_");

                let mut wb = WriteBatch::new();

                wb.put(
                    str_to_ascii_i8(&str).unwrap().as_slice(),
                    byte_nbt.as_slice(),
                );

                match self.db.write(WRITE_OPTIONS, wb) {
                    Ok(()) => Ok(()),
                    Err(dberr) => Err(WorldError::DBError(dberr)),
                }
            }
            Err(e) => Err(WorldError::NbtError(e)),
        }
    }
}

fn str_to_ascii_i8(s: &str) -> Result<Vec<i8>, &'static str> {
    if !s.is_ascii() {
        return Err("Input string contains non-ASCII characters");
    }

    let bytes = s.as_bytes();

    let ascii_i8: Vec<i8> = bytes.iter().map(|&b| b as i8).collect();

    Ok(ascii_i8)
}

fn vec_i8_into_u8(v: Vec<i8>) -> Vec<u8> {
    let mut v = std::mem::ManuallyDrop::new(v);

    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();

    unsafe { Vec::from_raw_parts(p as *mut u8, len, cap) }
}

fn vec_u8_into_i8(v: Vec<u8>) -> Vec<i8> {
    let mut v = std::mem::ManuallyDrop::new(v);

    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();

    unsafe { Vec::from_raw_parts(p as *mut i8, len, cap) }
}

impl Debug for WorldDB {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // can't be printed
        write!(f, "WorldDB(...)")
    }
}
