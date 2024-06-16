use proto_derive::ProtoCodec;

#[derive(Debug, Clone, ProtoCodec)]
pub struct Experiment {
    pub name: String,
    pub enabled: bool,
}
