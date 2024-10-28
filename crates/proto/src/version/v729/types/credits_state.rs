use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(u32)]
#[enum_endianness(var)]
pub enum CreditsState {
    Start = 0,
    Finished = 1,
}
