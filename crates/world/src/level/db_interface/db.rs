use crate::level::db_interface::bedrock_key::ChunkKey;
use crate::level::db_interface::config::{read_options, write_options};
use crate::level::db_interface::key_level::KeyTypeTag;
use crate::level::file_interface::{DatabaseBatchHolder, RawWorldTrait};
use crate::types::buffer_slide::{BetterCursor, SlideBuffer};
use bedrockrs_core::Vec2;
use bedrockrs_shared::world::dimension::Dimension;
use mojang_leveldb::error::DBError;
use mojang_leveldb::*;
use std::collections::HashSet;
use std::io::BufRead;
use std::marker::PhantomData;

pub struct LevelDBInterface<UserState> {
    db: DB,
    phantom_data: PhantomData<UserState>,
}

pub trait LevelDBKey {
    fn estimate_size(&self) -> usize;
    fn write_key(&self, buffer: &mut SlideBuffer<Vec<u8>>);
}

impl<UserState> LevelDBInterface<UserState> {
    pub fn new(path: &str, create_if_missing: bool) -> Result<Self, DBError> {
        let opts = Options {
            compression: CompressionType::ZlibRaw,
            create_if_missing,
        };
        let db = DB::open(path, opts)?;
        Ok(Self {
            db,
            phantom_data: PhantomData,
        })
    }

    pub fn get(
        &mut self,
        key: &impl LevelDBKey,
        options: ReadOptions,
    ) -> Result<Option<LevelDBManagedBytes>, DBError> {
        self.db.get(options, &Self::build_key(key))
    }

    pub fn set(
        &mut self,
        key: &impl LevelDBKey,
        options: WriteOptions,
        data: &[u8],
    ) -> Result<(), DBError> {
        let mut batch = WriteBatch::new();
        batch.put(&Self::build_key(key), data);
        self.db.write(options, batch)
    }

    pub fn build_key(key: &impl LevelDBKey) -> Vec<u8> {
        let mut key_bytes: Vec<u8> = vec![0; key.estimate_size()];
        let mut buff = SlideBuffer::new(&mut key_bytes);
        key.write_key(&mut buff);
        key_bytes
    }

    pub fn write_batch(&mut self, batch_buffer: Vec<(ChunkKey, Vec<u8>)>) -> Result<(), DBError> {
        let mut wb = WriteBatch::new();
        let mut keys: Vec<Vec<u8>> = Vec::new();
        for (key, raw) in &batch_buffer {
            keys.push(Self::build_key(key));
            wb.put(&keys[keys.len() - 1], &raw);
        }
        self.db.write(write_options(), wb)
    }

    pub fn write_key(&mut self, batch_buffer: Vec<ChunkKey>) -> Result<(), DBError> {
        let mut wb = WriteBatch::new();
        let mut keys: Vec<Vec<u8>> = Vec::new();
        for (key) in &batch_buffer {
            keys.push(Self::build_key(key));
            wb.put(&keys[keys.len() - 1], &[]);
        }
        self.db.write(write_options(), wb)
    }

    pub fn exists(&mut self, key: &impl LevelDBKey) -> Result<bool, DBError> {
        let data = self.get(key, read_options())?;
        Ok(data.is_some())
    }
}

impl<UserState> RawWorldTrait for LevelDBInterface<UserState> {
    type Err = DBError;
    type UserState = UserState;

    fn set_subchunk_raw(
        &mut self,
        chunk_info: ChunkKey,
        chunk_bytes: &[u8],
        _: &mut Self::UserState,
    ) -> Result<(), Self::Err> {
        self.set(&chunk_info, write_options(), chunk_bytes)
    }

    fn get_subchunk_raw(
        &mut self,
        chunk_info: ChunkKey,
        _: &mut Self::UserState,
    ) -> Result<Option<Vec<u8>>, Self::Err> {
        let information = self.get(&chunk_info, read_options())?;
        if information.is_none() {
            Ok(None)
        } else {
            Ok(Some(Vec::from(information.unwrap().get())))
        }
    }

    fn chunk_exists(
        &mut self,
        chunk_info: ChunkKey,
        _: &mut Self::UserState,
    ) -> Result<bool, Self::Err> {
        self.exists(&chunk_info)
    }

    fn write_subchunk_batch(
        &mut self,
        subchunk_batch_info: Vec<(ChunkKey, Vec<u8>)>,
        _: &mut Self::UserState,
    ) -> Result<(), Self::Err> {
        self.write_batch(subchunk_batch_info)
    }

    fn write_subchunk_marker_batch(
        &mut self,
        subchunk_batch_info: Vec<ChunkKey>,
        _: &mut Self::UserState,
    ) -> Result<(), Self::Err> {
        self.write_key(subchunk_batch_info)
    }

    fn exist_chunk(
        &mut self,
        chunk_info: ChunkKey,
        _: &mut Self::UserState,
    ) -> Result<(), Self::Err> {
        self.set(&chunk_info, write_options(), &[])
    }

    fn build_key(key: &ChunkKey) -> Vec<u8> {
        Self::build_key(key)
    }

    fn new(
        path: &str,
        create_if_missing: bool,
        _: &mut Self::UserState,
    ) -> Result<Self, Self::Err> {
        Self::new(path, create_if_missing)
    }

    fn generated_chunks(
        &mut self,
        _: &mut Self::UserState,
    ) -> Result<HashSet<(Dimension, Vec2<i32>)>, Self::Err> {
        let mut out_set = HashSet::new();
        for (key, _) in self.db.iter(read_options()) {
            if key.len() != 9 && key.len() != 13 {
                continue;
            }
            if KeyTypeTag::from_byte(*key.get().get(8).unwrap_or(&255)) == Some(KeyTypeTag::Version)
            {
                let mut buff = BetterCursor::new(key.get());
                let x = buff
                    .read::<i32>()
                    .ok_or(DBError::Unknown("Failed To Read X From Key".into()))?;
                let y = buff
                    .read::<i32>()
                    .ok_or(DBError::Unknown("Failed To Read Y From Key".into()))?;
                out_set.insert((Dimension::Overworld, (x, y).into()));
            } else if KeyTypeTag::from_byte(*key.get().get(12).unwrap_or(&255))
                == Some(KeyTypeTag::Version)
            {
                let mut buff = BetterCursor::new(key.get());
                let x = buff
                    .read::<i32>()
                    .ok_or(DBError::Unknown("Failed To Read X From Key".into()))?;
                let y = buff
                    .read::<i32>()
                    .ok_or(DBError::Unknown("Failed To Read Y From Key".into()))?;
                out_set.insert((
                    // This is actually a safe unwrap since we read from index 12 above we know all indexes before 12 must also exist
                    Dimension::from(buff.read::<i32>().unwrap()),
                    (x, y).into(),
                ));
            }
        }
        Ok(out_set)
    }
}

unsafe impl<UserState> Send for LevelDBInterface<UserState> {}
unsafe impl<UserState> Sync for LevelDBInterface<UserState> {}
