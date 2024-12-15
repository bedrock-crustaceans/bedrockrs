use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
#[enum_repr(i32)]
#[enum_endianness(var)]
pub enum HudVisibility {
    Hide = 0,
    Reset = 1,
    Count = 2,
}