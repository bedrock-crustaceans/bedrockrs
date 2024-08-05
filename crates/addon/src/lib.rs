use std::path::Path;

use crate::error::AddonError;
use crate::manifest::AddonManifest;

pub mod behavior;
pub mod error;
pub mod identifier;
pub mod language;
pub mod manifest;
pub mod resource;
pub mod version;

pub trait Addon {
    /// Returns the manifest of a given addon
    fn manifest(&self) -> &AddonManifest;

    /// Imports an addon from a given path
    fn import(path: impl AsRef<Path>) -> Result<Self, AddonError>
    where
        Self: Sized;

    /// Exports an addon to a given path
    fn export(path: impl AsRef<Path>) -> Result<Self, AddonError>
    where
        Self: Sized;

    /// Merges multiple addons into one
    /// The first addon builds the base and every addon following overwrites certain parts of it
    fn merge(addons: Vec<Self>) -> Self where Self: Sized;
}
