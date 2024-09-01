use bedrockrs_core::int::VAR;
use bedrockrs_proto_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 103)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ServerSettingsResponsePacket {
    pub form_id: VAR<u32>,
    pub form_json: String,
}
