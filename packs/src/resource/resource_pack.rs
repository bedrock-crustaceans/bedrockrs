use std::collections::HashMap;
use std::path::Path;
use image::RgbaImage;
use uuid::{Uuid, Version};

use crate::error::PackError;
use crate::language::Language;
use crate::pack::Pack;

pub struct ResourcePack {
    format_version: Version,
    name: String,
    description: String,
    icon: Option<RgbaImage>,
    uuid: Uuid,
    version: Version,
    min_engine_version: Version,
    languages: HashMap<String, Language>,
}

impl Pack for ResourcePack {
    fn name(&self) -> &str {
        &self.name
    }
    fn description(&self) -> &str {
        &self.description
    }
    fn uuid(&self) -> &Uuid {
        &self.uuid
    }
    fn version(&self) -> &Version {
        &self.version
    }

    fn import(path: impl AsRef<Path>) -> Result<Self, PackError>
    where
        Self: Sized
    {
        todo!()
    }

    fn export(path: impl AsRef<Path>) -> Result<Self, PackError>
    where
        Self: Sized
    {
        todo!()
    }
}
