use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::behavior::menu_category::AddonMenuCategory;
use crate::identifier::AddonIdentifier;

mod components;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddonItem {
    /// Specifies the version of the game this entity was made in. If the specified version is lower than the current version, any changes made to the entity in the vanilla version will be applied to it.
    format_version: String,
    /// Item definition, which includes the "description" and "components" sections.
    #[serde(rename = "minecraft:item")]
    definition: AddonItemDefinition,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddonItemDefinition {
    /// Description is a list of characters representing an item. The description MUST contain an identifier; other fields are optional.
    description: AddonItemDescription,
    /// List of all components used in this item. View the list of components in Item Components List.
    components: HashMap<AddonIdentifier, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddonItemDescription {
    /// The identifier for this item; must include a namespace. The 'Minecraft' namespace must not be used, unless overriding a Vanilla item.
    identifier: AddonIdentifier,
    /// The creative group name and category for this item.
    menu_category: Option<AddonMenuCategory>,
}
