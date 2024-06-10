use crate::compression::Compression;

pub trait LoginProvider {
    fn get_compression(&self) -> Compression;
}
