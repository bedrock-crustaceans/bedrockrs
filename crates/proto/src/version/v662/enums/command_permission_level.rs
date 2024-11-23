use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum CommandPermissionLevel {
    Any = 0,
    GameDirectors = 1,
    Admin = 2,
    Host = 3,
    Owner = 4,
    Internal = 5,
}