use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 87)]
#[derive(ProtoCodec)]
pub struct StopSoundPacket {
    pub sound_name: String,
    pub stop_all_sounds: bool,
}