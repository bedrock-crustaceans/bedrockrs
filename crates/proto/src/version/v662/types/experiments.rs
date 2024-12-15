use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub struct Experiment {
    pub name: String,
    pub enabled: bool,
}

#[derive(ProtoCodec)]
pub struct Experiments {
    #[vec_repr(u32)]
    #[vec_endianness(le)]
    pub experiments: Vec<Experiment>, 
    pub ever_toggled: bool,
}