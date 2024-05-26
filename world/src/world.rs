use std::{collections::HashMap, path::PathBuf};
use mojang_leveldb::{error::DBError, Options, ReadOptions, WriteBatch, WriteOptions, DB};
use nbt::{endian::little_endian::NbtLittleEndian, NbtTag};
use uuid::Uuid;

use crate::{error::WorldError, str_to_ascii_i8, vec_i8_into_u8, vec_u8_into_i8};

pub struct World {
    db: DB
}

const READ_OPTIONS: ReadOptions = ReadOptions{fill_cache: true, verify_checksums: true};
const WRITE_OPTIONS: WriteOptions = WriteOptions{sync: true};

impl World {
    pub fn open(directory: PathBuf) -> Result<World, DBError> {
        Ok(World { 
            db: DB::open(
                &directory.join("db").display().to_string(), 
                Options{
                    compression: mojang_leveldb::CompressionType::ZlibRaw, 
                    create_if_missing: true}
            )?
        })
    }

    pub fn get_player(&self, uuid: Uuid) -> Result<Option<HashMap<String, NbtTag>>, WorldError> {

        let mut str = uuid.to_string(); 
        str.insert_str(0, "player_"); 

        let res = self.db.get(
            READ_OPTIONS, 
            str_to_ascii_i8(&str).unwrap().as_slice()
        );

        if let Ok(Some(bytes)) = res {
            let u8_bytes = vec_i8_into_u8(bytes.get().into());
            let res = NbtTag::nbt_deserialize_vec::<NbtLittleEndian>(u8_bytes);
            if let Ok(tag) = res {
                if let NbtTag::Compound(ctag) = tag.1 {
                    return Ok(Some(ctag));
                }  
            } else if let Err(e) = res {
                return Err(WorldError::NbtError(e));
            }
        } else if let Err(x) = res {
            return Err(WorldError::DBError(x));
        }

        Ok(None)

    }

    pub fn set_player(&mut self, uuid: Uuid, tag: NbtTag) -> Result<(), WorldError> {
        match tag.nbt_serialize_vec::<NbtLittleEndian>("") {
            Ok(sertag) => {
                let byte_nbt = vec_u8_into_i8(sertag);

                let mut str = uuid.to_string(); 
                str.insert_str(0, "player_"); 

                let mut wb = WriteBatch::new();

                wb.put(str_to_ascii_i8(&str).unwrap().as_slice(), byte_nbt.as_slice());

                match self.db.write(WRITE_OPTIONS, wb) {
                    Ok(()) => Ok(()),
                    Err(dberr) => Err(WorldError::DBError(dberr))
                }
            },
            Err(e) => {
                Err(WorldError::NbtError(e))
            }
        }
        
    }
}