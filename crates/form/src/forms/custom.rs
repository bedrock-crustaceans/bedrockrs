use serde::{Deserialize, Serialize};
use crate::elems::Element;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CustomForm {
    pub title: String,
    #[serde(rename = "content")]
    pub elements: Vec<Element>,
}
