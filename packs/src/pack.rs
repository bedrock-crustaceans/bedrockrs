use std::path::Path;

use uuid::Uuid;

use crate::error::PackError;

pub trait Pack {
    fn open(path: impl AsRef<Path>) -> Result<Self, PackError>
    where
        Self: Sized;

    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn uuid(&self) -> &Uuid;
}
