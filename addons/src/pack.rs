use std::path::Path;

use bedrockrs_core::SemVer;
use uuid::Uuid;

use crate::error::PackError;

pub trait Pack {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn uuid(&self) -> &Uuid;
    fn version(&self) -> &SemVer;

    fn import(path: impl AsRef<Path>) -> Result<Self, PackError>
    where
        Self: Sized;

    fn export(path: impl AsRef<Path>) -> Result<Self, PackError>
    where
        Self: Sized;
}
