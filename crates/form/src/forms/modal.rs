use crate::elems::button::Button;
use serde::{Deserialize, Serialize};

/// [`ModalForm`] represents a modal form.
/// These forms consist of a body containing text and two [`Buttons`](Button) at the bottom,
/// usually labeled "Yes" (`gui.yes` for automatic translation) and "No" (`gui.no` for automatic translation)
/// While the button text can be customized,
/// unlike a [`SimpleForm`](crate::forms::SimpleForm), they cannot include images next to them.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModalForm {
    pub title: String,
    #[serde(rename = "content")]
    pub body: String,
    pub button1: Button,
    pub button2: Button,
}
