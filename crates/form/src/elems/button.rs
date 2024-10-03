use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Button {
    pub text: String,
    pub image: Option<ButtonImage>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum ButtonImage {
    Path(String),
    Url(String),
}
