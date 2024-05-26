use std::path::PathBuf;
use mojang_leveldb::{error::DBError, Options, ReadOptions, WriteBatch, WriteOptions, DB};
use nbt::{endian::little_endian::NbtLittleEndian, error::NbtError, NbtTag};
use uuid::Uuid;

use crate::{str_to_ascii_i8, vec_i8_into_u8, vec_u8_into_i8};

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

    pub fn get_player(&self, uuid: Uuid) -> Result<Option<Result<(String, NbtTag), NbtError>>, DBError> {

        let mut str = uuid.to_string(); 
        str.insert_str(0, "player_"); 

        let res = self.db.get(
            READ_OPTIONS, 
            str_to_ascii_i8(&str).unwrap().as_slice()
        )?;

        if let Some(bytes) = res {
            let u8_bytes = vec_i8_into_u8(bytes.get().into());
            return Ok(Some(NbtTag::nbt_deserialize_vec::<NbtLittleEndian>(u8_bytes)))
        }

        Ok(None)

    }

    pub fn set_player(&mut self, uuid: Uuid, tag_name: String, tag: NbtTag) -> Result<Result<(), DBError>, NbtError> {
        let byte_nbt = vec_u8_into_i8(tag.nbt_serialize_vec::<NbtLittleEndian>(tag_name)?);

        let mut str = uuid.to_string(); 
        str.insert_str(0, "player_"); 

        let mut wb = WriteBatch::new();

        wb.put(str_to_ascii_i8(&str).unwrap().as_slice(), byte_nbt.as_slice());

        Ok(self.db.write(WRITE_OPTIONS, wb))
    }
}