use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 135)]
#[derive(ProtoCodec)]
pub struct ClientCacheBlobStatusPacket {
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub missing_blobs: Vec<u64>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub obtained_blobs: Vec<u64>,
}

// TODO: custom proto impl, both Vec lengths come first in packet, then both their elements, respectfully. (elements are le)