use std::path::Path;
use crate::error::PackError;

pub struct Language(Vec<String>);

impl Language {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, PackError> {
        todo!()
    }
}
