use bedrockrs_core::int::VAR;
use bedrockrs_proto_derive::ProtoCodec;
use crate::types::modal_form_cancel_reason::ModalFormCancelReason;

#[derive(ProtoCodec, Debug, Clone)]
pub struct ModelFormResponsePacket {
    form_id: VAR<u32>,
    form_response: Option<String>,
    cancel_reason: Option<ModalFormCancelReason>,
}
