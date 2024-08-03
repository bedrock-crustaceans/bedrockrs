use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddonMenuCategory {
    /// Determines which category this block/item will be placed under in the inventory and crafting table container screens. Options are "construction", "nature", "equipment", "items", and "none". If omitted or "none" is specified, the block/item will not appear in the inventory or crafting table container screens.
    category: String,
    /// Specifies the language file key that maps to which expandable/collapsible group this block/item will be a part of within a category. If this field is omitted, or there is no group whose name matches the loc string, this block/item will be placed standalone in the given category.
    group: Option<String>,
    /// Determines whether this block/item can be used with commands. Commands can use blocks/items by default, but you may use this to disable that functionality.
    hidden_in_commands: Option<bool>,
}
