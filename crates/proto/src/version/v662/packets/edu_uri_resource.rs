use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::EduSharedUriResource;

#[gamepacket(id = 170)]
#[derive(ProtoCodec)]
pub struct EduUriResourcePacket {
    pub edu_shared_uri_resource: EduSharedUriResource,
}