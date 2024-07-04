use proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct GameRule {
    name: String,
    editable: bool,
    // TODO figure out what the value can be, the proto docs are fucked up and say that it is 3 varints
    value: u8
}
