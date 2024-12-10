use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum ServerAuthMovementMode {
    ClientAuthoritative = 0,
    ServerAuthoritative = 1,
    ServerAuthoritativeWithRewind = 2,
}