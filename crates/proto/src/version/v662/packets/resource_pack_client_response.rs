use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 8)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ResourcePackClientResponsePacket {
    pub response: ResourcePackResponse,
    #[vec_repr(u16)]
    #[vec_endianness(le)]
    pub downloading_packs: Vec<String>
}

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(i8)]
pub enum ResourcePackResponse {
    Cancel = 1,
    Downloading = 2,
    DownloadingFinished = 3,
    ResourcePackStackFinished = 4,
}
