use serde_json::json;

use crate::elems::Element;
use crate::error::FormError;
use crate::window::Form;

pub struct CustomForm {
    pub title: String,
    pub elements: Vec<Box<dyn Element + Send>>,
}

impl Form for CustomForm {
    fn form_serialize(&self) -> String {
        let mut elems_strings = vec![];

        for elem in &self.elements {
            elems_strings.push(elem.elem_serialize());
        }

        json!({
            "type": "custom_form",
            "title": self.title,
            "content": elems_strings,
        })
        .to_string()
    }

    fn form_deserialize(form_json: &str) -> Result<Self, FormError> {
        todo!()
    }
}
