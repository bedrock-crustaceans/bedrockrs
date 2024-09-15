use thiserror::Error;

#[derive(Error, Debug)]
pub enum WorldError {
    #[error("DB Error: {0}")]
    DBError(#[from] mojang_leveldb::error::DBError),

    #[error("Nbt Error: {0}")]
    NbtError(#[from] nbtx::NbtError),

    #[error("Format Error: {0}")]
    FormatError(String),
}
