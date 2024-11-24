use crate::types::buffer_slide::SlideBuffer;

pub trait LevelDBKey {
    fn estimate_size(&self) -> usize;
    fn write_key(&self, buffer: &mut SlideBuffer<Vec<u8>>);
}
