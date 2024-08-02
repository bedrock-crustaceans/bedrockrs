use crate::error::FormError;

pub mod custom;
pub mod modal;
pub mod simple;

pub trait Form {
    fn form_serialize(&self) -> String;
    fn form_deserialize(form_json: &str) -> Result<Self, FormError>
    where
        Self: Sized;
}
