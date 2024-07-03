use std::collections::HashMap;
use std::path::Path;
use bedrock_core::SemVer;
use image::RgbaImage;
use uuid::{Uuid, Version};

use crate::error::PackError;
use crate::language::{LanguageValues, Languages};
use crate::pack::Pack;

#[derive(Debug, Clone)]
pub struct ResourcePack {
    format_version: SemVer,
    name: String,
    description: String,
    icon: Option<RgbaImage>,
    uuid: Uuid,
    version: SemVer,
    min_engine_version: SemVer,
    languages: Languages,
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
