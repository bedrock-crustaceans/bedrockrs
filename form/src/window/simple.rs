use serde_json::json;

use crate::elems::button::Button;
use crate::elems::Element;
use crate::window::Form;

pub struct SimpleForm {
    pub title: String,
    pub body: String,
    pub buttons: Vec<Button>,
}

impl Form for SimpleForm {
    fn form_serialize(&self) -> String {
        let mut buttons_strings = vec![];

        for button in &self.buttons {
            buttons_strings.push(button.elem_serialize());
        }

        json!({
            "type": "form",
            "title": self.title,
            "content": self.body,
            "buttons": buttons_strings
        })
        .to_string()
    }

    fn form_deserialize(form_json: &str) -> Self {
        todo!()
    }
}
