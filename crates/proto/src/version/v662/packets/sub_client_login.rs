use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 94)]
#[derive(ProtoCodec)]
pub struct SubClientLoginPacket {
    pub connection_request: String, // TODO: SubClientConnectionRequest diagram, not sure.
}