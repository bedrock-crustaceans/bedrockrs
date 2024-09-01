use bedrockrs_proto_macros::ProtoCodec;
use bedrockrs_shared::actor_unique_id::ActorUniqueID;
use bedrockrs_shared::world::permissions_level::PermissionLevel;

#[derive(ProtoCodec, Debug, Clone)]
pub struct AbilityData {
    /// This field is not necessary, 0 seems to work.
    target_player_raw_id: ActorUniqueID,
    permission: PermissionLevel,
}
