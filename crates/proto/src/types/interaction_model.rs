use bedrockrs_core::int::VAR;
use bedrockrs_proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(VAR::<u32>)]
pub enum InteractionModel {
    Touch = 0,
    Crosshair = 1,
    Classic = 2,
}
