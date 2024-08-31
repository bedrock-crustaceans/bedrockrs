use bedrockrs_proto_derive::ProtoCodec;
use bedrockrs_core::int::VAR;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(VAR::<u32>)]
pub enum PermissionLevel {
    /// Any/Normal permission
    Default = 0,
    Operator = 1,
    Admin = 2,
    Host = 3,
    Owner = 4,
}
