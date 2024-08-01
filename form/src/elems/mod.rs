use serde_json::Value;
use crate::error::FormError;

pub mod button;
pub mod dropdown;
pub mod input;
pub mod label;
pub mod slider;
pub mod step_slider;
pub mod toggle;

pub trait Element {
    fn elem_serialize(&self) -> Value;
    fn elem_deserialize(elem_json: Value) -> Result<Self, FormError> where Self: Sized;
}
