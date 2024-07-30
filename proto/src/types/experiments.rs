use bedrockrs_core::int::LE;
use bedrockrs_proto_core::ProtoCodec;
use bedrockrs_proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct Experiments {
    #[len_repr(LE::<u32>)]
    pub experiments: Vec<Experiment>,
    pub ever_toggled: bool,
}

#[derive(ProtoCodec, Debug, Clone)]
pub struct Experiment {
    pub name: String,
    pub enabled: bool,
}