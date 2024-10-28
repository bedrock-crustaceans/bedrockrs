use bedrockrs_macros::{gamepacket, ProtoCodec};

use crate::version::v729::types::resource_packs_response_status::ResourcePacksResponseStatus;

#[gamepacket(id = 8)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ResourcePacksResponsePacket {
    pub response: ResourcePacksResponseStatus,
    /// The addons that are downloaded/getting downloaded
    /// with their pack name as strings
    #[vec_repr(u16)]
    #[vec_endianness(le)]
    pub downloading_packs: Vec<String>,
}
