use serde::{Deserialize, Serialize};

/// [`Label`] represents a static text box on a form.
/// It is used purely for displaying text and does not accept any player input.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Label {
    /// Represents the [`Label's`](Label) content, which may include Minecraft formatting codes.
    pub text: String,
}
