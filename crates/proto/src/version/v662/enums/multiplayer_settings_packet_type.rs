use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum MultiplayerSettingsPacketType {
    EnableMultiplayer = 0,
    DisableMultiplayer = 1,
    RefreshJoincode = 2,
}