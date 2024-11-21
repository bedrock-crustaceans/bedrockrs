use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum HudVisibility {
    Hide = 0,
    Reset = 1,
    Count = 2,
}