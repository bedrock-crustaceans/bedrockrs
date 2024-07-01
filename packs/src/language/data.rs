use std::collections::HashMap;
use std::path::Path;
use crate::error::PackError;

pub struct LanguageData(HashMap<String, String>);

impl LanguageData {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, PackError> {
        todo!()
    }
}
