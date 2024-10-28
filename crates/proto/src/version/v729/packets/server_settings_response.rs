use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 103)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ServerSettingsResponsePacket {
    #[endianness(var)]
    pub form_id: u32,
    pub form_json: String,
}
