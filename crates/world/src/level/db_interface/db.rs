use std::io::Cursor;

pub trait LevelDBKey {
    fn estimate_size(&self) -> usize;
    fn write_key(&self, buffer: &mut Cursor<&mut [u8]>);
}
