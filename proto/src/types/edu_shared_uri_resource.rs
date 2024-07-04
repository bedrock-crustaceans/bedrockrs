use proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct EduSharedResourceUri {
    button_name: String,
    link_uri: String
}