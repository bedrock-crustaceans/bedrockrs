use serde::{Deserialize, Serialize};

/// A combination of DropDown and Slider
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StepSlider {
    pub text: String,
    #[serde(rename = "steps")]
    pub options: Vec<String>,
    pub default: i32,
}
