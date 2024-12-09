use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 190)]
#[derive(ProtoCodec)]
pub struct EditorNetworkPacket {
    #[nbt]
    pub binary_payload: nbtx::Value, // TODO: NBT Structure
}