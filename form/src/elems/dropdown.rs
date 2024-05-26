use serde_json::{json, Value};

use crate::elems::Element;

pub struct DropDown {
    text: String,
    options: Vec<String>,
    default_index: i32,
}

impl Element for DropDown {
    fn elem_serialize(&self) -> Value {
        json!({
            "type": "dropdown",
            "text": self.text,
            "default": self.default_index,
            "options": self.options,
        })
    }
}
