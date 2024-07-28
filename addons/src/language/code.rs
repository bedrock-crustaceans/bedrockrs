use std::fmt::{Debug, Formatter};
use bedrockrs_core::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum LanguageCode {
    VanillaCode(String),
    CustomCode(Vec2<String>),
}

impl Debug for LanguageCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LanguageCode::VanillaCode(v) => { write!(f, "VanillaCode({v})") }
            LanguageCode::CustomCode(v) => { write!(f, "CustomCode([{}, {}])", v.x, v.y) }
        }
    }
}
