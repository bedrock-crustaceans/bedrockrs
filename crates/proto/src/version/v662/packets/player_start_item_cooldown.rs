use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 176)]
#[derive(ProtoCodec)]
pub struct PlayerStartItemCooldownPacket {
    pub item_category: String,
    #[endianness(var)]
    pub duration_ticks: i32,
}