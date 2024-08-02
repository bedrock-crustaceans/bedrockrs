use bedrockrs_core::int::LE;
use bedrockrs_proto_core::ProtoCodec;
use bedrockrs_proto_derive::ProtoCodec;

use crate::types::resource_packs_response_status::ResourcePacksResponseStatus;

#[derive(ProtoCodec, Debug, Clone)]
pub struct ResourcePacksResponsePacket {
    pub response: ResourcePacksResponseStatus,
    /// The addons that are downloaded/getting downloaded
    /// with their pack name as strings
    #[len_repr(LE::<u16>)]
    pub downloading_packs: Vec<String>,
}
