use bedrockrs_core::int::VAR;
use bedrockrs_proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct ModalFormRequestPacket {
    pub form_id: VAR<u32>,
    pub form_json: String,
}
