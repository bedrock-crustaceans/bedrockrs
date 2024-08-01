use bedrockrs_proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub enum ModalFormCancelReason {
    Closed = 0,
    Buys = 1,
}
