use crate::version::v662::types::ScoreboardId;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
struct IdentityInfoUpdateEntry {
    pub scoreboard_id: ScoreboardId,
    #[endianness(var)]
    pub player_unique_id: i64,
}

#[derive(ProtoCodec)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum ScoreboardIdentityPacketType {
    Update {
        #[vec_repr(u32)]
        #[vec_endianness(var)]
        identity_info: Vec<IdentityInfoUpdateEntry>,
    } = 0,
    Remove {
        #[vec_repr(u32)]
        #[vec_endianness(var)]
        identity_info: Vec<ScoreboardId>,
    } = 1,
}