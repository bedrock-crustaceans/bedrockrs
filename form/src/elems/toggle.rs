use serde_json::{json, Value};

use crate::elems::Element;

pub struct Toggle {
    text: String,
    default: bool,
}

impl Element for Toggle {
    fn elem_serialize(&self) -> Value {
        json!({
            "type": "toggle",
            "text": self.text,
            "default": self.default,
        })
    }
}
