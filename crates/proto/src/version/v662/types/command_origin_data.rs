use bedrockrs_macros::ProtoCodec;
use uuid::Uuid;

#[derive(ProtoCodec)]
pub struct CommandOriginData {
    #[endianness(var)]
    pub command_type: u32,
    pub command_uuid: Uuid,
    pub request_id: String,
}