use serde_json::{json, Value};
use crate::elems::Element;

/// A combination of DropDown and Slider
pub struct StepSlider {
    text: String,
    options: Vec<String>,
    default_index: i32,
}

impl Element for StepSlider {
    fn elem_serialize(&self) -> Value {
        json!({
            "type": "step_slider",
		    "text": self.text,
		    "default": self.default_index,
		    "steps": self.options,
        })
    }
}