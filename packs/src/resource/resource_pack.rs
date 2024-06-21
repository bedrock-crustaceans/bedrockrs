use std::path::Path;

use uuid::Uuid;

use crate::error::PackError;
use crate::pack::Pack;

pub struct ResourcePack {
    name: String,
    description: String,
    uuid: Uuid,
}

impl Pack for ResourcePack {
    fn load(path: impl AsRef<Path>) -> Result<Self, PackError>
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
