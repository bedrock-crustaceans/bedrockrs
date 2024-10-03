use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Slider {
    pub text: String,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub default: f64,
}
