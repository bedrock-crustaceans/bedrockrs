use std::collections::HashMap;
use std::path::PathBuf;

use crate::behavior::blocks::AddonBlock;
use crate::language::Languages;
use crate::manifest::AddonManifest;

mod blocks;

#[derive(Debug, Clone)]
pub struct BehaviorPack {
    manifest: AddonManifest,
    languages: Languages,
    blocks: HashMap<PathBuf, AddonBlock>,
    functions: HashMap<PathBuf, String>,
}
