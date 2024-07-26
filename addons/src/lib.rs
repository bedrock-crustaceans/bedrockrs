use std::fmt::Debug;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::AddonError;
use crate::manifest::AddonManifest;

pub mod behavior;
pub mod error;
mod identifier;
pub mod language;
pub mod manifest;
pub mod resource;
mod version;

pub trait Addon {
    fn manifest(&self) -> &AddonManifest;

    fn import(path: impl AsRef<Path>) -> Result<Self, AddonError>
    where
        Self: Sized;

    fn export(path: impl AsRef<Path>) -> Result<Self, AddonError>
    where
        Self: Sized;
}
