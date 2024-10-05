use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct GameRule {
    pub name: String,
    pub editable: bool,
    pub value: GameRuleValue,
}

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(u32)]
#[enum_endianness(var)]
#[repr(u32)]
pub enum GameRuleValue {
    Bool(bool) = 1,
    VarU32(#[endianness(var)] u32) = 2,
    F32(#[endianness(le)] f32) = 3,
}
