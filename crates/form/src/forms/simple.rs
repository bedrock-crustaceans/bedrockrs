use serde::{Deserialize, Serialize};
use crate::elems::button::Button;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimpleForm {
    pub title: String,
    #[serde(rename = "content")]
    pub body: String,
    pub buttons: Vec<Button>,
}
