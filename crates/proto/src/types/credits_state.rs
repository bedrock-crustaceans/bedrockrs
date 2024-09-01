use bedrockrs_core::int::VAR;
use bedrockrs_proto_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(VAR::<u32>)]
pub enum CreditsState {
    Start = 0,
    Finished = 1,
}
