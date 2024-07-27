use std::collections::HashMap;
use std::{fs, io};
use std::fs::FileType;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use json_comments::CommentSettings;
use walkdir::WalkDir;
use crate::Addon;
use crate::behavior::blocks::AddonBlock;
use crate::behavior::items::AddonItem;
use crate::error::AddonError;
use crate::error::AddonError::{IOError, JsonError};
use crate::language::Languages;
use crate::manifest::AddonManifest;

mod blocks;
mod items;
mod menu_category;

#[derive(Debug, Clone)]
pub struct BehaviorPack {
    manifest: AddonManifest,
    //languages: Languages,
    blocks: HashMap<PathBuf, AddonBlock>,
    items: HashMap<PathBuf, AddonItem>,
    functions: HashMap<PathBuf, String>,
    scripts: HashMap<PathBuf, String>,
}

impl Addon for BehaviorPack {
    fn manifest(&self) -> &AddonManifest {
        &self.manifest
    }

    fn import(path: impl AsRef<Path>) -> Result<Self, AddonError>
    where
        Self: Sized
    {
        let path = path.as_ref().to_path_buf();

        // Manifest
        let manifest_path = path.join("manifest.json");
        let manifest = fs::read_to_string(&manifest_path).map_err(|e| IOError(Arc::new(e), manifest_path.clone()))?;
        let manifest: AddonManifest = serde_json::from_str(&manifest).map_err(|e| JsonError(Arc::new(e), manifest_path))?;

        // Languages
        // TODO: Impl langauge parsing
        //let languages = Languages::import(path.join("blocks"))?;

        // Blocks
        let blocks_path = path.join("blocks");
        let mut blocks = HashMap::new();

        'blocks_walk: for blocks_entry in WalkDir::new(&blocks_path)
            .into_iter()
            .filter(|v| { if let Ok(v) = v { v.file_type().is_file() } else { false } }) {

            if let Ok(entry) = blocks_entry {
                let block_path = entry.path();

                let block = fs::read_to_string(block_path)
                    .map_err(|e| IOError(Arc::new(e), block_path.to_path_buf()))?;

                // If the file is empty skip it
                if block.trim().is_empty() {
                    continue 'blocks_walk;
                }

                // Strip all c-style comments
                let stripper = CommentSettings::c_style().strip_comments(block.as_bytes());

                let block: AddonBlock = serde_json::from_reader(stripper)
                    .map_err(|e| JsonError(Arc::new(e), block_path.to_path_buf()))?;

                blocks.insert(block_path.to_path_buf(), block);
            }
        }

        // Items
        let items_path = path.join("items");
        let mut items = HashMap::new();

        'items_walk: for items_entry in WalkDir::new(&items_path)
            .into_iter()
            .filter(|v| { if let Ok(v) = v { v.file_type().is_file() } else { false } }) {

            if let Ok(entry) = items_entry {
                let item_path = entry.path();

                let item = fs::read_to_string(item_path)
                    .map_err(|e| IOError(Arc::new(e), item_path.to_path_buf()))?;

                // If the file is empty skip it
                if item.trim().is_empty() {
                    continue 'items_walk;
                }

                // Strip all c-style comments
                let stripper = CommentSettings::c_style().strip_comments(item.as_bytes());

                let item: AddonItem = serde_json::from_reader(stripper)
                    .map_err(|e| JsonError(Arc::new(e), item_path.to_path_buf()))?;

                items.insert(item_path.to_path_buf(), item);
            }
        }

        // Functions
        let functions_path = path.join("functions");
        let mut functions = HashMap::new();

        for functions_entry in WalkDir::new(&functions_path)
            .into_iter()
            .filter(|v| { if let Ok(v) = v { v.file_type().is_file() } else { false } }) {

            if let Ok(entry) = functions_entry {
                let function_path = entry.path();

                let function = fs::read_to_string(function_path)
                    .map_err(|e| IOError(Arc::new(e), function_path.to_path_buf()))?;

                functions.insert(function_path.to_path_buf(), function);
            }
        }

        // Functions
        let scripts_path = path.join("scripts");
        let mut scripts = HashMap::new();

        for scripts_entry in WalkDir::new(&scripts_path)
            .into_iter()
            .filter(|v| { if let Ok(v) = v { v.file_type().is_file() } else { false } }) {

            if let Ok(entry) = scripts_entry {
                let script_path = entry.path();

                let script = fs::read_to_string(script_path)
                    .map_err(|e| IOError(Arc::new(e), script_path.to_path_buf()))?;

                scripts.insert(script_path.to_path_buf(), script);
            }
        }

        Ok(Self {
            manifest,
            //languages,
            blocks,
            items,
            functions,
            scripts
        })
    }

    fn export(path: impl AsRef<Path>) -> Result<Self, AddonError>
    where
        Self: Sized
    {
        todo!()
    }
}