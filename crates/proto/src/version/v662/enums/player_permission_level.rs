use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum PlayerPermissionLevel {
    Visitor = 0,
    Member = 1,
    Operator = 2,
    Custom = 3,
}