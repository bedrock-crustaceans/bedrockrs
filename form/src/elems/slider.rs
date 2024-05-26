use serde_json::{json, Value};

use crate::elems::Element;

pub struct Slider {
    text: String,
    min: f64,
    max: f64,
    step_size: f64,
    default: f64,
}

impl Element for Slider {
    fn elem_serialize(&self) -> Value {
        json!({
            "type": "slider",
            "text": self.text,
            "min": self.min,
            "max": self.max,
            "step": self.step_size,
            "default": self.default,
        })
    }
}
