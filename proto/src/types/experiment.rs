use proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct Experiment {
    pub name: String,
    pub enabled: bool,
}
