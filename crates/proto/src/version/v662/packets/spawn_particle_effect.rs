use crate::version::v662::types::{ActorUniqueID, MolangVariableMap};
use bedrockrs_core::Vec3;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 118)]
#[derive(ProtoCodec)]
pub struct SpawnParticleEffectPacket {
    pub dimension_id: i8,
    pub actor_id: ActorUniqueID,
    #[endianness(le)]
    pub position: Vec3<f32>,
    pub effect_name: String,
    pub molang_variables: Option<MolangVariableMap>
}