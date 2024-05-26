use serde_json::Value;

pub mod label;
pub mod button;
pub mod input;
pub mod slider;
pub mod toggle;
pub mod step_slider;
pub mod dropdown;

pub trait Element {
    fn elem_serialize(&self) -> Value;
}