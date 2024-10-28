use bedrockrs_macros::{gamepacket, ProtoCodec};
use xuid::Xuid;

use crate::version::v729::types::title_type::TitleType;

#[gamepacket(id = 88)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct SetTitlePacket {
    pub title_type: TitleType,
    pub title_text: String,
    #[endianness(var)]
    pub fade_in_time: i32,
    #[endianness(var)]
    pub stay_time: i32,
    #[endianness(var)]
    pub fade_out_time: i32,
    pub xuid: Xuid,
    pub platform_online_id: String,
}
