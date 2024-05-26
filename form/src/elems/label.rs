use serde_json::{json, Value};
use crate::elems::Element;

pub struct Label {
    text: String
}

impl Element for Label {
    fn elem_serialize(&self) -> Value {
        json!({
            "type": "label",
            "text": self.text,
        })
    }
}