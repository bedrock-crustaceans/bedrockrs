pub mod values;

use std::collections::HashMap;
use std::path::Path;
pub use values::*;
use crate::error::PackError;

pub struct Languages(HashMap<String, LanguageValues>);

impl Languages {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, PackError> {
        todo!()
    }

    pub fn languages(&self) -> Vec<String> {
        self.0.keys().map(|f| {
            f.clone()
        }).collect()
    }
}