use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 100)]
#[derive(ProtoCodec)]
pub struct ModalFormRequestPacket {
    #[endianness(var)]
    pub form_id: u32,
    pub form_ui_json: String,
}