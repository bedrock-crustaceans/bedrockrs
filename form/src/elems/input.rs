use serde_json::{json, Value};

use crate::elems::Element;
use crate::error::FormError;

pub struct Input {
    pub text: String,
    pub default: String,
    pub placeholder: String,
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

    fn elem_deserialize(elem_json: Value) -> Result<Self, FormError> {
        todo!()
    }
}
