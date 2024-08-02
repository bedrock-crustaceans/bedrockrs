use serde_json::{json, Value};

use crate::elems::Element;
use crate::error::FormError;

pub struct Slider {
    pub text: String,
    pub min: f64,
    pub max: f64,
    pub step_size: f64,
    pub default: f64,
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

    fn elem_deserialize(elem_json: Value) -> Result<Self, FormError> {
        todo!()
    }
}
