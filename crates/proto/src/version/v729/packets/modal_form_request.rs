use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 100)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ModalFormRequestPacket {
    #[endianness(var)]
    pub form_id: u32,
    pub form_json: String,
}
