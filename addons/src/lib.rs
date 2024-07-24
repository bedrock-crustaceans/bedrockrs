use std::path::Path;

use bedrockrs_core::Vec3;
use serde::{Deserialize, Serialize};

use crate::error::AddonError;
use crate::manifest::AddonManifest;

pub mod behavior;
pub mod error;
pub mod language;
mod manifest;
pub mod resource;

pub trait Addon {
    fn manifest(&self) -> &AddonManifest;

    fn import(path: impl AsRef<Path>) -> Result<Self, AddonError>
    where
        Self: Sized;

    fn export(path: impl AsRef<Path>) -> Result<Self, AddonError>
    where
        Self: Sized;
}

/// A version used in Addons that is either a Vector [a, b, c] or SemVer String.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AddonDynamicVersion {
    Vector(Vec3<u32>),
    SemVer(semver::Version),
}
