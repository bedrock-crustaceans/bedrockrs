use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum ModalFormCancelReason {
    UserClosed = 0,
    UserBusy = 1,
}
