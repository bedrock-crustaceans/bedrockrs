use serialize_derive::{MCProtoDeserialize, MCProtoSerialize};

#[derive(Debug, MCProtoSerialize, MCProtoDeserialize)]
pub struct Experiment {
    pub name: String,
    pub enabled: bool,
}