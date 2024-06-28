use bedrock_core::LE;
use proto_core::ProtoCodec;
use proto_derive::ProtoCodec;

use crate::types::experiment::Experiment;

#[derive(ProtoCodec, Debug, Clone)]
pub struct Experiments {
    #[len_type(LE::< u32 >)]
    pub experiments: Vec<Experiment>,
    pub ever_toggled: bool,
}
