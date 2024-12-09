use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 165)]
#[derive(ProtoCodec)]
pub struct SyncActorPropertyPacket {
    #[nbt]
    pub property_data: nbtx::Value, // TODO: NBT Structure
}