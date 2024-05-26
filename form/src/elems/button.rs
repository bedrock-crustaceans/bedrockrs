use serde_json::{json, Value};
use crate::elems::Element;

pub enum ButtonImage {
    Path(String),
    Url(String)
}

pub struct Button {
    pub text: String,
    pub image: Option<ButtonImage>
}

impl Element for Button {
    fn elem_serialize(&self) -> Value {
        let (button_type, image_data) = match &self.image {
            None => { ("path", "") }
            Some(v) => {
                match v {
                    ButtonImage::Path(v) => { ("path", v.as_str()) }
                    ButtonImage::Url(v) => { ("url", v.as_str()) }
                }
            }
        };

        json!({
            "type": button_type,
            "text": self.text,
            "data": image_data,
        })
    }
}