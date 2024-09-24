use crate::error::FormError;
use serde_json::Value;
use crate::elems::button::Button;
use crate::elems::dropdown::Dropdown;
use crate::elems::input::Input;
use crate::elems::label::Label;
use crate::elems::slider::Slider;
use crate::elems::step_slider::StepSlider;
use crate::elems::toggle::Toggle;

pub mod button;
pub mod dropdown;
pub mod input;
pub mod label;
pub mod slider;
pub mod step_slider;
pub mod toggle;

pub trait Element {
    fn elem_serialize(&self) -> Value;
    fn elem_deserialize(elem_json: Value) -> Result<Self, FormError>
    where
        Self: Sized;
}

pub enum Elements {
    Button(Button),
    Dropdown(Dropdown),
    Input(Input),
    Label(Label),
    Slider(Slider),
    StepSlider(StepSlider),
    Toggle(Toggle),
}

impl Elements {
    pub fn elem_serialize(&self) -> Value {
        
    }

    fn elem_deserialize(elem_json: Value) -> Result<Self, FormError>
    where
        Self: Sized {
        
    }
}
