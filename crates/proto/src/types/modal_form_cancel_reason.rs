use bedrockrs_core::int::LE;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(LE::<u8>)]
pub enum ModalFormCancelReason {
    Closed = 0,
    Busy = 1,
}
