use std::path::Path;

use uuid::{Uuid, Version};

use crate::error::PackError;
use crate::pack::Pack;

pub struct ResourcePack {
    format_version: Version,
    name: String,
    description: String,
    uuid: Uuid,
    min_engine_version: Version,
}

impl Pack for ResourcePack {
    fn open(path: impl AsRef<Path>) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn name(&self) -> &str {
        &self.name
    }
    fn description(&self) -> &str {
        &self.description
    }
    fn uuid(&self) -> &Uuid {
        &self.uuid
    }
}
