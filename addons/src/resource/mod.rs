use std::path::Path;

use crate::error::AddonError;
use crate::language::Languages;
use crate::manifest::AddonManifest;
use crate::Addon;

#[derive(Debug, Clone)]
pub struct ResourcePack {
    manifest: AddonManifest,
    languages: Languages,
}

impl Addon for ResourcePack {
    fn manifest(&self) -> &AddonManifest {
        &self.manifest
    }

    fn import(path: impl AsRef<Path>) -> Result<Self, AddonError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn export(path: impl AsRef<Path>) -> Result<Self, AddonError>
    where
        Self: Sized,
    {
        todo!()
    }
}
