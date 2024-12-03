use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::ResourcePackResponse;

#[gamepacket(id = 8)]
#[derive(ProtoCodec)]
pub struct ResourcePackClientResponsePacket {
    pub response: ResourcePackResponse,
    #[vec_repr(u16)]
    #[vec_endianness(le)]
    pub downloading_packs: Vec<String>,
}