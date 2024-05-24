use proto_derive::{ProtoCodec};

#[derive(Debug, ProtoCodec)]
pub struct Experiment {
    pub name: String,
    pub enabled: bool,
}
