use bedrockrs_core::int::VAR;
use bedrockrs_proto_macros::{gamepacket, ProtoCodec};
use bedrockrs_shared::actor_runtime_id::ActorRuntimeID;
use uuid::Uuid;

#[gamepacket(id = 152)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct EmoteListPacket {
    runtime_id: ActorRuntimeID,
    #[len_repr(VAR::<u32>)]
    emote_piece_ids: Vec<Uuid>,
}
