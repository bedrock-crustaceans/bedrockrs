use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::behavior::menu_category::AddonMenuCategory;
use crate::identifier::AddonIdentifier;

pub mod components;
pub mod traits;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddonBlock {
    /// Specifies the version of the game this block was made in. If the version is lower than the current version, any changes made to the entity in the vanilla version will be applied to it.
    pub format_version: String,
    #[serde(rename = "minecraft:block")]
    pub definition: AddonBlockDefinition,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddonBlockDefinition {
    /// List of block characteristics that are applicable to all permutations of the block. The description MUST contain an identifier; the other fields are optional. View the other fields in Block Description.
    pub description: AddonBlockDescription,
    /// List of all components used in this block. View the list of components in Block Components List.
    pub components: HashMap<AddonIdentifier, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddonBlockDescription {
    /// The identifier for this block. The name must include a namespace and must not use the Minecraft namespace unless overriding a Vanilla block.
    pub identifier: AddonIdentifier,
    /// Map of key/value pairs that maps the state name (key) to an array of all possible values for that state (value). Learn how to use block states in Block States and Permutations.
    pub states: Option<HashMap<String, AddonBlockState>>,
    /// Specifies the menu category and group for the block, which determine where this block is placed in the inventory and crafting table container screens. If this field is omitted, the block will not appear in the inventory or crafting table container screens.
    pub menu_category: Option<AddonMenuCategory>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddonBlockState(pub Vec<serde_json::Value>);
