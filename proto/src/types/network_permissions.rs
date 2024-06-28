use proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct NetworkPermissions {
    server_auth_sound_enabled: bool,
}
