use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub struct EduSharedUriResource {
    pub button_name: String,
    pub link_uri: String,
}