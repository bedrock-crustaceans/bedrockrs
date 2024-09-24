use serde_json::{json, Value};

use crate::elems::Element;
use crate::error::FormError;

pub struct Dropdown {
    pub text: String,
    pub options: Vec<String>,
    pub default_index: i32,
}

impl Element for Dropdown {
    fn elem_serialize(&self) -> Value {
        json!({
            "type": "dropdown",
            "text": self.text,
            "default": self.default_index,
            "options": self.options,
        })
    }

    fn elem_deserialize(elem_json: Value) -> Result<Self, FormError> {
        todo!()
    }
}
