use std::collections::HashMap;
use std::path::Path;

use uuid::{Uuid, Version};

use crate::error::PackError;
use crate::language::Language;
use crate::pack::Pack;

pub struct BehaviorPack {
    format_version: Version,
    name: String,
    description: String,
    uuid: Uuid,
    version: Version,
    min_engine_version: Version,
    languages: HashMap<String, Language>,
}

impl Pack for BehaviorPack {
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
