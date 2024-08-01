use bedrockrs_proto_derive::ProtoCodec;
use bedrockrs_core::int::LE;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(LE::<u8>)]
pub enum ModalFormCancelReason {
    Closed = 0,
    Busy = 1,
}
