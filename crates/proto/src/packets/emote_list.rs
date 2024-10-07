use bedrockrs_macros::{gamepacket, ProtoCodec};
use bedrockrs_shared::actor_runtime_id::ActorRuntimeID;
use uuid::Uuid;

#[gamepacket(id = 152)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct EmoteListPacket {
    pub runtime_id: ActorRuntimeID,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub emote_piece_ids: Vec<Uuid>,
}
