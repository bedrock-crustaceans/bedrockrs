use bedrockrs_proto_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct NetworkPermissions {
    pub server_auth_sound_enabled: bool,
}
