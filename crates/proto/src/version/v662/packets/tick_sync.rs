use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 23)]
#[derive(ProtoCodec)]
pub struct TickSyncPacket {
     #[endianness(le)]
     pub client_request_timestamp: i64,
     #[endianness(le)]
     pub server_response_timestamp: i64,
}