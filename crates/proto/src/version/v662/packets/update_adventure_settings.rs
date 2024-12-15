use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::AdventureSettings;

#[gamepacket(id = 188)]
#[derive(ProtoCodec)]
pub struct UpdateAdventureSettingsPacket {
    pub adventure_settings: AdventureSettings,
}