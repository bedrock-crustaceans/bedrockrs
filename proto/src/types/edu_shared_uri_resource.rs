use bedrockrs_proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct EduSharedResourceUri {
    pub button_name: String,
    pub link_uri: String,
}
