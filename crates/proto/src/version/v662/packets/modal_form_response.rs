use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::ModalFormCancelReason;

#[gamepacket(id = 101)]
#[derive(ProtoCodec)]
pub struct ModalFormResponsePacket {
    #[endianness(var)]
    pub form_id: u32,
    pub json_response: Option<String>, // TODO: might not be string, dumb proto docs list this as std::optional<class Json::Value>
    pub form_cancel_reason: Option<ModalFormCancelReason> // TODO: enum_repr
}