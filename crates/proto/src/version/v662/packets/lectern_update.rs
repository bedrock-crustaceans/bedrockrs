use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::NetworkBlockPosition;

#[gamepacket(id = 125)]
#[derive(ProtoCodec)]
pub struct LecternUpdatePacket {
    pub new_page_to_show: i8,
    pub total_pages: i8,
    pub position_of_lectern_to_update: NetworkBlockPosition,
}