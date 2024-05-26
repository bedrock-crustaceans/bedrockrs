use serde_json::Value;

pub mod button;
pub mod dropdown;
pub mod input;
pub mod label;
pub mod slider;
pub mod step_slider;
pub mod toggle;

pub trait Element {
    fn elem_serialize(&self) -> Value;
}
