use bedrockrs_core::int::VAR;
use bedrockrs_proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct ModelFormRequestPacket {
    form_id: VAR<u32>,
    form_json: String,
}
