use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::ActorUniqueID;

#[gamepacket(id = 182)]
#[derive(ProtoCodec)]
pub struct ChangeMobPropertyPacket {
    pub actor_id: ActorUniqueID,
    pub property_name: String,
    pub bool_component_value: bool,
    pub string_component_value: String,
    #[endianness(var)]
    pub int_component_value: i32,
    #[endianness(le)]
    pub float_component_value: f32,
}