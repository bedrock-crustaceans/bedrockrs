use std::fmt::{Debug, Formatter};
use std::path::Path;

use bedrockrs_core::Vec3;
use serde::{Deserialize, Serialize};

use crate::error::AddonError;
use crate::manifest::AddonManifest;

pub mod behavior;
pub mod error;
pub mod language;
pub mod manifest;
pub mod resource;
mod version;
mod identifier;

pub trait Addon {
    fn manifest(&self) -> &AddonManifest;

    fn import(path: impl AsRef<Path>) -> Result<Self, AddonError>
    where
        Self: Sized;

    fn export(path: impl AsRef<Path>) -> Result<Self, AddonError>
    where
        Self: Sized;
}

