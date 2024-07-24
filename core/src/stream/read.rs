use std::io::Cursor;

pub type ByteStreamRead<'a> = Cursor<&'a Vec<u8>>;
