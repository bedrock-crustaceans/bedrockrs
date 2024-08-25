use bedrockrs_core::{int::LE, Vec2, Vec3};
use bedrockrs_shared::{actor_runtime_id::ActorRuntimeID, actor_unique_id::ActorUniqueID};

use crate::{
    connection::ConnectionShard,
    error::LoginError,
    gamepacket::GamePackets,
    packets::add_actor_packet::AddActorPacket,
    types::{
        actor_type::ActorType,
        property_sync_data::{FloatEntriesList, IntEntriesList, PropertySyncData},
    },
};

use super::provider::LoginProviderServer;

pub async fn add_actor(
    conn: &mut ConnectionShard,
    provider: &mut impl LoginProviderServer,
) -> Result<(), LoginError> {
    //////////////////////////////////////
    // todo: AddActorPacket
    //////////////////////////////////////

    let add_actor = AddActorPacket {
        target_actor_id: ActorUniqueID(610),
        target_runtime_id: ActorRuntimeID(403),
        actor_type: ActorType::Pig.to_string(),
        position: Vec3 {
            x: LE::new(4.0),
            y: LE::new(8.0),
            z: LE::new(7.0),
        },
        velocity: Vec3 {
            x: LE::new(4.0),
            y: LE::new(8.0),
            z: LE::new(7.0),
        },
        rotation: Vec2 {
            x: LE::new(270.0),
            y: LE::new(90.0),
        },
        y_head_rotation: LE::new(45.0),
        y_body_rotation: LE::new(90.0),
        attributes: vec![],
        actor_data: vec![],
        synched_properties: PropertySyncData {
            int: IntEntriesList { entries: vec![] },
            float: FloatEntriesList { entries: vec![] },
        },
        actor_links: vec![],
    };

    conn.send(GamePackets::AddEntity(add_actor))
        .await
        .map_err(|e| LoginError::ConnectionError(e))?;

    conn.flush()
        .await
        .map_err(|e| LoginError::ConnectionError(e))?;

    Ok(())
}
