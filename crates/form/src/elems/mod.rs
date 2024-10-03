use crate::elems::dropdown::Dropdown;
use crate::elems::input::Input;
use crate::elems::label::Label;
use crate::elems::slider::Slider;
use crate::elems::step_slider::StepSlider;
use crate::elems::toggle::Toggle;
use serde::{Deserialize, Serialize};

pub mod button;
pub mod dropdown;
pub mod input;
pub mod label;
pub mod slider;
pub mod step_slider;
pub mod toggle;

/// An enum of all possible [`Elements`](Element) for a [`CustomForm`](crate::forms::custom::CustomForm).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum Element {
    Dropdown(Dropdown),
    Input(Input),
    Label(Label),
    Slider(Slider),
    StepSlider(StepSlider),
    Toggle(Toggle),
}
