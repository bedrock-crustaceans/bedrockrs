use bedrockrs_macros::ProtoCodec;
use crate::version::v662::enums::IdentityDefinition;
use crate::version::v662::types::ScoreboardId;

#[derive(ProtoCodec)]
struct ScorePacketInfoChangeEntry {
    pub id: ScoreboardId,
    pub objective_name: String,
    #[endianness(le)]
    pub score_value: i32,
}

#[derive(ProtoCodec)]
struct ScorePacketInfoRemoveEntry {
    pub id: ScoreboardId,
    pub objective_name: String,
    #[endianness(le)]
    pub score_value: i32,
    pub identity_definition_type: IdentityDefinition::Type,
}

#[derive(ProtoCodec)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum ScorePacketType {
    Change {
        #[vec_repr(u32)]
        #[vec_endianness(var)]
        score_packet_info: Vec<ScorePacketInfoChangeEntry>,
    } = 0,
    Remove {
        #[vec_repr(u32)]
        #[vec_endianness(var)]
        score_packet_info: Vec<ScorePacketInfoRemoveEntry>,
    } = 1,
}