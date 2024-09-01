use bedrockrs_proto_derive::ProtoCodec;
use bedrockrs_core::int::VAR;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(VAR::<u32>)]
pub enum CreditsState {
    Start = 0,
    Finished = 1,
}
