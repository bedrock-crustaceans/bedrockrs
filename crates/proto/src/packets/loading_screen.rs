use crate::types::loading_screen_action::LoadingScreenAction;
use bedrockrs_core::int::LE;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 312)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct LoadingScreenPacket {
    pub screen_action: LoadingScreenAction,
    pub screen_id: Option<LE<u32>>,
}
