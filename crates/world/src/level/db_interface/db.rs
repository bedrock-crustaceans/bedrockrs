use std::marker::PhantomData;
use crate::level::db_interface::config::{read_options, write_options};
use crate::types::buffer_slide::SlideBuffer;
use mojang_leveldb::error::DBError;
use mojang_leveldb::*;
use crate::level::db_interface::bedrock_key::ChunkKey;
use crate::level::file_interface::{DatabaseBatchHolder, RawWorldTrait};

pub struct LevelDBInterface<UserState> {
    db: DB,
    phantom_data: PhantomData<UserState>
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
        Ok(Self { db, phantom_data: PhantomData })
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
        let mut key_bytes: Vec<u8> = Vec::with_capacity(key.estimate_size());
        unsafe {
            key_bytes.set_len(key.estimate_size()); // Spooky unsafe
        }
        let mut buff = SlideBuffer::new(&mut key_bytes);
        key.write_key(&mut buff);
        key_bytes
    }

    pub fn write_batch(&mut self, batch_buffer: Vec<DatabaseBatchHolder>) -> Result<(), DBError> {
        let mut wb = WriteBatch::new();
        for batch_info in &batch_buffer {
            wb.put(batch_info.key(), batch_info.data());
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

    fn set_subchunk_raw(&mut self, chunk_info: ChunkKey, chunk_bytes: &[u8], _: &mut Self::UserState) -> Result<(), Self::Err> {
        self.set(&chunk_info, write_options(), chunk_bytes)
    }

    fn get_subchunk_raw(&mut self, chunk_info: ChunkKey, _: &mut Self::UserState) -> Result<Option<Vec<u8>>, Self::Err> {
        let information = self.get(&chunk_info, read_options())?;
        if information.is_none() {
            Ok(None)
        } else {
            Ok(Some(Vec::from(information.unwrap().get())))
        }
    }

    fn chunk_exists(&mut self, chunk_info: ChunkKey, _: &mut Self::UserState) -> Result<bool, Self::Err> {
        self.exists(&chunk_info)
    }

    fn write_subchunk_batch(&mut self, subchunk_batch_info: Vec<DatabaseBatchHolder>, _: &mut Self::UserState) -> Result<(), Self::Err> {
        self.write_batch(subchunk_batch_info)
    }

    fn exist_chunk(&mut self, chunk_info: ChunkKey, _: &mut Self::UserState) -> Result<(), Self::Err> {
        self.set(&chunk_info, write_options(), &[])
    }

    fn build_key(key: &ChunkKey) -> Vec<u8> {
        Self::build_key(key)
    }

    fn new(path: &str, create_if_missing: bool, _: &mut Self::UserState) -> Result<Self, Self::Err> {
        Self::new(path, create_if_missing)
    }

    fn generated_chunks(&mut self, state: &mut Self::UserState) -> Result<Vec<ChunkKey>, Self::Err> {
        todo!()
        //FIXME: Need to fork mojang-leveldb and add support for iterating the keys
    }
}

unsafe impl<UserState> Send for LevelDBInterface<UserState> {}
unsafe impl<UserState> Sync for LevelDBInterface<UserState> {}
