use uuid::Uuid;
use bedrockrs_macros::ProtoCodec;
use crate::version::v662::enums::BuildPlatform;
use crate::version::v662::types::{ActorUniqueID, SerializedSkin};

#[derive(ProtoCodec)]
struct AddPlayerListEntry {
    pub uuid: Uuid,
    pub target_actor_id: ActorUniqueID,
    pub player_name: String,
    pub xbl_xuid: String,
    pub platform_chat_id: String,
    pub build_platform: BuildPlatform,
    pub serialized_skin: SerializedSkin,
    pub is_teacher: bool,
    pub is_host: bool,
    pub is_sub_client: bool,
    
}

#[derive(ProtoCodec)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum PlayerListPacketType {
    Add {
        #[vec_repr(u32)]
        #[vec_endianness(var)]
        add_player_list: Vec<AddPlayerListEntry>,
        is_trusted_skin: bool,
    } = 0,
    Remove {
        #[vec_repr(u32)]
        #[vec_endianness(var)]
        remove_player_list: Vec<Uuid>
    } = 1,
}