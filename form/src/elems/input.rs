use serde_json::{json, Value};

use crate::elems::Element;

pub struct Input {
    text: String,
    default: String,
    placeholder: String,
}

impl Element for Input {
    fn elem_serialize(&self) -> Value {
        json!({
            "type": "input",
            "text": self.text,
            "default": self.default,
            "placeholder": self.placeholder,
        })
    }
}
