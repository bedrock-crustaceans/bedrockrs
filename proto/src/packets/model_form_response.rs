use bedrockrs_core::int::VAR;
use bedrockrs_proto_derive::ProtoCodec;
use crate::types::modal_form_cancel_reason::ModalFormCancelReason;

#[derive(ProtoCodec, Debug, Clone)]
pub struct ModelFormResponsePacket {
    pub form_id: VAR<u32>,
    pub form_response: Option<String>,
    pub cancel_reason: Option<ModalFormCancelReason>,
}
