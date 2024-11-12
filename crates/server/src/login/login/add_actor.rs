use bedrockrs_core::{Vec2, Vec3};
use bedrockrs_shared::{actor_runtime_id::ActorRuntimeID, actor_unique_id::ActorUniqueID};

use crate::{
    connection::ConnectionShard,
    error::LoginError,
    gamepackets::GamePackets,
    packets::add_actor::AddActorPacket,
    types::{
        actor_type::ActorType,
        property_sync_data::{FloatEntriesList, IntEntriesList, PropertySyncData},
    },
};

use super::provider::LoginProvider;

pub async fn add_actor(
    conn: &mut ConnectionShard,
    provider: &mut impl LoginProvider,
) -> Result<(), LoginError> {
    //////////////////////////////////////
    // todo: AddActorPacket
    //////////////////////////////////////

    let add_actor = AddActorPacket {
        target_actor_id: ActorUniqueID(610),
        target_runtime_id: ActorRuntimeID(403),
        actor_type: ActorType::Pig.to_string(),
        position: Vec3 {
            x: 4.0,
            y: 8.0,
            z: 7.0,
        },
        velocity: Vec3 {
            x: 4.0,
            y: 8.0,
            z: 7.0,
        },
        rotation: Vec2 { x: 270.0, y: 90.0 },
        y_head_rotation: 45.0,
        y_body_rotation: 90.0,
        attributes: vec![],
        actor_data: vec![],
        synced_properties: PropertySyncData {
            int: IntEntriesList { entries: vec![] },
            float: FloatEntriesList { entries: vec![] },
        },
        actor_links: vec![],
    };

    conn.send(GamePackets::AddEntity(add_actor)).await?;
    conn.flush().await?;

    Ok(())
}
