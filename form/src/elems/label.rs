use serde_json::{json, Value};

use crate::elems::Element;
use crate::error::FormError;

pub struct Label {
    text: String,
}

impl Element for Label {
    fn elem_serialize(&self) -> Value {
        json!({
            "type": "label",
            "text": self.text,
        })
    }

    fn elem_deserialize(elem_json: Value) -> Result<Self, FormError> {
        todo!()
    }
}
