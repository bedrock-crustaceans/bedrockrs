use mojang_leveldb::{ReadOptions, WriteOptions};

pub fn read_options() -> ReadOptions {
    ReadOptions {
        fill_cache: true,
        verify_checksums: false,
    }
}

pub fn write_options() -> WriteOptions {
    WriteOptions { sync: false }
}
