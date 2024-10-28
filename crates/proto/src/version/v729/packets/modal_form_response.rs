use crate::version::v729::types::modal_form_cancel_reason::ModalFormCancelReason;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 101)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ModalFormResponsePacket {
    #[endianness(var)]
    pub form_id: u32,
    pub form_response: Option<String>,
    pub cancel_reason: Option<ModalFormCancelReason>,
}
