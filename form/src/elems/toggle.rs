use serde_json::{json, Value};

use crate::elems::Element;
use crate::error::FormError;

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

    fn elem_deserialize(elem_json: Value) -> Result<Self, FormError> {
        todo!()
    }
}
