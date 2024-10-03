use crate::elems::button::Button;
use serde::{Deserialize, Serialize};

/// [`SimpleForm`] represents a form consisting of a title,
/// body, and a set of buttons beneath the body.
/// These [`Buttons`](Button) can optionally include images alongside them.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimpleForm {
    /// Refers to the title.
    pub title: String,
    /// Refers to the body.
    #[serde(rename = "content")]
    pub body: String,
    /// Refers to all available buttons. Sequence is maintained.
    pub buttons: Vec<Button>,
}
