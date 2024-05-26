use serde_json::json;

use crate::elems::Element;
use crate::forms::Form;

pub struct CustomForm {
    title: String,
    elements: Vec<Box<dyn Element>>,
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
}
