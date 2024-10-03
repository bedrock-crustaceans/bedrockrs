use crate::elems::button::Button;
use serde::{Deserialize, Serialize};

/// [`SimpleForm`] represents a form consisting of a title,
/// body, and a set of buttons beneath the body.
/// These [`Buttons`](Button) can optionally include images alongside them.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimpleForm {
    pub title: String,
    #[serde(rename = "content")]
    pub body: String,
    pub buttons: Vec<Button>,
}
