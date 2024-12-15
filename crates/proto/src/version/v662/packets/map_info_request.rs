use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::ActorUniqueID;

#[derive(ProtoCodec)]
struct ClientPixelsListEntry {
    #[endianness(le)]
    pub pixel: u32,
    #[endianness(le)]
    pub index: u16,
}

#[gamepacket(id = 68)]
#[derive(ProtoCodec)]
pub struct MapInfoRequestPacket {
    pub map_unique_id: ActorUniqueID,
    #[vec_repr(u32)]
    #[vec_endianness(le)]
    pub client_pixels_list: Vec<ClientPixelsListEntry>
}