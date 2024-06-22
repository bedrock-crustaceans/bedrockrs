use serde_json::json;

use crate::elems::button::Button;
use crate::elems::Element;
use crate::window::Form;

pub struct ModalForm {
    pub title: String,
    pub body: String,
    pub button1: Button,
    pub button2: Button,
}

impl Form for ModalForm {
    fn form_serialize(&self) -> String {
        json!({
            "type": "form",
            "title": self.title,
            "content": self.body,
            "button1": self.button1.elem_serialize(),
            "button2": self.button2.elem_serialize(),
        })
            .to_string()
    }
}
