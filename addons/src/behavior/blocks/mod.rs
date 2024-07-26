use std::collections::HashMap;

use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

use crate::identifier::AddonIdentifier;

mod components;
mod traits;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddonBlock {
    /// Specifies the version of the game this block was made in. If the version is lower than the current version, any changes made to the entity in the vanilla version will be applied to it.
    format_version: semver::Version,
    #[serde(rename = "minecraft:block")]
    definition: AddonBlockDefinition,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddonBlockDefinition {
    /// List of block characteristics that are applicable to all permutations of the block. The description MUST contain an identifier; the other fields are optional. View the other fields in Block Description.
    description: AddonBlockDescription,
    /// List of all components used in this block. View the list of components in Block Components List.
    components: HashMap<AddonIdentifier, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddonBlockDescription {
    /// The identifier for this block. The name must include a namespace and must not use the Minecraft namespace unless overriding a Vanilla block.
    identifier: AddonIdentifier,
    /// Map of key/value pairs that maps the state name (key) to an array of all possible values for that state (value). Learn how to use block states in Block States and Permutations.
    states: Option<Vec<AddonBlockState>>,
    menu_category: Option<AddonBlockMenuCategory>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddonBlockState {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddonBlockMenuCategory {
    /// Determines which category this block will be placed under in the inventory and crafting table container screens. Options are "construction", "nature", "equipment", "items", and "none". If omitted or "none" is specified, the block will not appear in the inventory or crafting table container screens.
    category: String,
    /// Specifies the language file key that maps to which expandable/collapsible group this block will be a part of within a category. If this field is omitted, or there is no group whose name matches the loc string, this block will be placed standalone in the given category.
    group: String,
    /// Determines whether this block can be used with commands. Commands can use blocks by default, but you may use this to disable that functionality.
    hidden_in_commands: bool,
}
