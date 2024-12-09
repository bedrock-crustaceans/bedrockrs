use crate::version::v662::types::EduSharedUriResource;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 170)]
#[derive(ProtoCodec)]
pub struct EduUriResourcePacket {
    pub edu_shared_uri_resource: EduSharedUriResource,
}