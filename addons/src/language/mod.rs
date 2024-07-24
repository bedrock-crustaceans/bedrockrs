use std::collections::HashMap;
use std::path::Path;

pub use values::*;

use crate::error::AddonError;

pub mod values;

#[derive(Debug, Clone)]
pub struct Languages(HashMap<String, LanguageValues>);

impl Languages {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, AddonError> {
        todo!()
    }

    pub fn languages(&self) -> Vec<String> {
        self.0.keys().map(|f| f.clone()).collect()
    }
}
