use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Copy, Clone, Eq, PartialEq)]
#[enum_repr(i32)]
#[enum_endianness(var)]
pub enum TitleType {
    Clear = 0,
    Reset = 1,
    Title = 2,
    Subtitle = 3,
    Actionbar = 4,
    Times = 5,
    TitleTextObject = 6,
    SubtitleTextObject = 7,
    ActionbarTextObject = 8,
}
