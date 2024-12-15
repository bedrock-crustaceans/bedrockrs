use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::{ActorUniqueID, MolangVariableMap, Vec3};

#[gamepacket(id = 118)]
#[derive(ProtoCodec)]
pub struct SpawnParticleEffectPacket {
    pub dimension_id: i8,
    pub actor_id: ActorUniqueID,
    pub position: Vec3,
    pub effect_name: String,
    pub molang_variables: Option<MolangVariableMap>
}