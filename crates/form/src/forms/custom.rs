use crate::elems::Element;
use serde::{Deserialize, Serialize};

/// [`CustomForm`] represents a form that can be sent to a player,
/// containing fields that the player may fill out.
/// All possible fields are in the [`Element`] enum.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CustomForm {
    pub title: String,
    #[serde(rename = "content")]
    pub elements: Vec<Element>,
}
