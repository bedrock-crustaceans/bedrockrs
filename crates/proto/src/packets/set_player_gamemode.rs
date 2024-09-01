use bedrockrs_proto_macros::{gamepacket, ProtoCodec};
use bedrockrs_shared::world::gamemode::Gamemode;

#[gamepacket(id = 62)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct SetPlayerGamemode {
    gamemode: Gamemode,
}
