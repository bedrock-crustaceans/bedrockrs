use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct Experiments {
    #[vec_repr(u32)]
    #[vec_endianness(le)]
    experiments: Vec<ExperimentData>,
    previously_toggled: bool,
}

#[derive(ProtoCodec, Debug, Clone)]
pub struct ExperimentData {
    name: String,
    enabled: bool,
}
