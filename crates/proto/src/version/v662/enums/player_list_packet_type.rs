use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum PlayerListPacketType {
    Add = 0,
    Remove = 1,
}