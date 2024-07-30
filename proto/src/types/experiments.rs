use bedrockrs_core::int::LE;
use bedrockrs_proto_core::ProtoCodec;
use bedrockrs_proto_derive::ProtoCodec;

use crate::types::experiment::Experiment;

#[derive(ProtoCodec, Debug, Clone)]
pub struct Experiments {
    #[len_repr(LE::<u32>)]
    pub experiments: Vec<Experiment>,
    pub ever_toggled: bool,
}
