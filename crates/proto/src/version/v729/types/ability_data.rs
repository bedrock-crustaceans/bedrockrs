use bedrockrs_macros::ProtoCodec;
use bedrockrs_shared::actor_unique_id::ActorUniqueID;
use bedrockrs_shared::world::permissions_level::PermissionLevel;

#[derive(ProtoCodec, Debug, Clone)]
pub struct AbilityData {
    /// This field is not necessary, 0 seems to work.
    pub target_player_id: ActorUniqueID,
    pub permission: PermissionLevel,
}
