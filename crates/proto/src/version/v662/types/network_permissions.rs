use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub struct NetworkPermissions {
    pub server_auth_sound_enabled: bool,
}