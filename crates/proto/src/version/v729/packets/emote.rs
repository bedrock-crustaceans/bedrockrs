use bedrockrs_macros::{gamepacket, ProtoCodec};
use bedrockrs_shared::actor_runtime_id::ActorRuntimeID;
use xuid::Xuid;

#[gamepacket(id = 138)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct EmotePacket {
    runtime_id: ActorRuntimeID,
    emote_id: String,
    /// Emote length measured in ticks.
    #[endianness(var)]
    emote_length: u32,
    xuid: Xuid,
    platform_id: String,
    // TODO: Turn this into an enum
    flags: i8,
}
