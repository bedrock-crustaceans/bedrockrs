use uuid::Uuid;
use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::SerializedSkin;

#[gamepacket(id = 93)]
#[derive(ProtoCodec)]
pub struct PlayerSkinPacket {
    pub uuid: Uuid,
    pub serialized_skin: SerializedSkin,
    pub new_skin_name: String,
    pub old_skin_name: String,
    pub trusted_marketplace_skin: bool,
}