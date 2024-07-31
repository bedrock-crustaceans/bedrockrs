use bedrockrs_proto_derive::ProtoCodec;
use bedrockrs_core::int::VAR;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(VAR::<u32>)]
pub enum InteractionModel {
    Touch = 0,
    Crosshair = 1,
    Classic = 2,
}
