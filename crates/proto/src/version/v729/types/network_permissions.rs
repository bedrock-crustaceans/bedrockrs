use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct NetworkPermissions {
    pub server_auth_sound: bool,
}
