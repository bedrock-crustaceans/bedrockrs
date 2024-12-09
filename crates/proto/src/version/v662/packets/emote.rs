use crate::version::v662::types::ActorRuntimeID;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[derive(ProtoCodec)]
#[enum_repr(i8)]
#[repr(i8)]
enum Flags {
    ServerSide = 1 << 0,
    MuteEmoteChat = 1 << 1,
}

#[gamepacket(id = 138)]
#[derive(ProtoCodec)]
pub struct EmotePacket {
    pub actor_runtime_id: ActorRuntimeID,
    pub emote_id: String,
    pub xuid: String,
    pub platform_id: String,
    pub flags: Flags,
}