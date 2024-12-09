use bedrockrs_macros::{gamepacket, ProtoCodec};

#[derive(ProtoCodec, Clone, Debug)]
struct MissingBlobEntry {
    #[endianness(le)]
    pub blob_id: u64,
    pub blob_data: String,
}

#[gamepacket(id = 136)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct ClientCacheMissResponsePacket {
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub missing_blobs: Vec<MissingBlobEntry>
}