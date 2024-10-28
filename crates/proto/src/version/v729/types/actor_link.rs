use bedrockrs_macros::ProtoCodec;
use bedrockrs_shared::actor_unique_id::ActorUniqueID;

use super::actor_link_type::ActorLinkType;

#[derive(ProtoCodec, Debug, Clone)]
pub struct ActorLink {
    pub actor_unique_id_a: ActorUniqueID,
    pub actor_unique_id_b: ActorUniqueID,
    pub link_type: ActorLinkType,
    pub immediate: bool,
    pub passenger_seat_id: bool,
}
