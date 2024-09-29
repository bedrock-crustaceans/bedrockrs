use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(i8)]
pub enum ModalFormCancelReason {
    Closed = 0,
    Busy = 1,
}
