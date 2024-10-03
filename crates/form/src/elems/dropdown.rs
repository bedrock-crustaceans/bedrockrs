use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Dropdown {
    pub text: String,
    pub options: Vec<String>,
    pub default: i32,
}

// impl Dropdown {
//     fn elem_serialize(&self) -> Value {
//         json!({
//             "type": "dropdown",
//             "text": self.text,
//             "default": self.default,
//             "options": self.options,
//         })
//     }
//
//     fn elem_deserialize(elem_json: Value) -> Result<Self, FormError> {
//         todo!()
//     }
// }
