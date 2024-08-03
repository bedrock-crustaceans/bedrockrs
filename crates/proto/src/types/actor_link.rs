use bedrockrs_shared::actor_unique_id::ActorUniqueID;

pub struct ActorLink {
    actor_unique_id_a: ActorUniqueID,
    actor_unique_id_b: ActorUniqueID,
    link_type: u8,
    immediate: bool,
    passenger_seat_id: bool,
}
