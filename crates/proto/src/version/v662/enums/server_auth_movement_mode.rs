use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum ServerAuthMovementMode {
    ClientAuthoritative = 0,
    ServerAuthoritative = 1,
    ServerAuthoritativeWithRewind = 2,
}