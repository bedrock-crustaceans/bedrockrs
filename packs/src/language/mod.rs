pub mod data;

use std::collections::HashMap;
use std::path::Path;
pub use data::*;
use crate::error::PackError;

pub struct Languages(HashMap<String, LanguageData>);

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