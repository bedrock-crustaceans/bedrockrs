use nbt::error::NbtError;
use thiserror::Error;

#[derive(Error)]
#[derive(Debug)]
pub enum WorldError {
    #[error("DB Error: {0}")]
    DBError(mojang_leveldb::error::DBError),

    #[error("Nbt Error: {0}")]
    NbtError(NbtError),

    #[error("Format Error: {0}")]
    FormatError(String)
}