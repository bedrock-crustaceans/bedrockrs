pub mod custom;
pub mod modal;
pub mod simple;

pub trait Form {
    fn form_serialize(&self) -> String;
}