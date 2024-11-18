use std::ops::IndexMut;

pub struct SlideBuffer<'a, Container: IndexMut<usize, Output = u8> + ?Sized> {
    container: &'a mut Container,
    index: usize,
}

impl<'a, Container: IndexMut<usize, Output = u8>> SlideBuffer<'a, Container> {
    pub fn new(container: &'a mut Container) -> Self {
        Self {
            container,
            index: 0,
        }
    }

    pub fn write<T: Sized + bytemuck::Pod>(&mut self, val: T) -> &mut Self {
        let bytes = bytemuck::bytes_of(&val);
        for byte in bytes {
            self.push_byte(*byte);
        }
        self
    }

    pub fn push_byte(&mut self, val: u8) {
        self.index += 1;
        self.container[self.index - 1] = val;
    }
}
