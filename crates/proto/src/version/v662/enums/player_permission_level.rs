use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum PlayerPermissionLevel {
    Visitor = 0,
    Member = 1,
    Operator = 2,
    Custom = 3,
}