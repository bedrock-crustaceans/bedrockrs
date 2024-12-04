use bedrockrs_macros::{gamepacket, ProtoCodec};

#[derive(ProtoCodec)]
struct MissingBlobEntry {
    #[endianness(le)]
    pub blob_id: u64,
    pub blob_data: String,
}

#[gamepacket(id = 136)]
#[derive(ProtoCodec)]
pub struct ClientCacheMissResponsePacket {
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub missing_blobs: Vec<MissingBlobEntry>
}