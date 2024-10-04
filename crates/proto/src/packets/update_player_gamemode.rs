use bedrockrs_macros::{gamepacket, ProtoCodec};
use bedrockrs_shared::world::gamemode::Gamemode;

#[gamepacket(id = 62)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct UpdatePlayerGamemode {
    pub gamemode: Gamemode,
}
