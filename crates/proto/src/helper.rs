use bedrockrs_proto_core::GamePacketsAll;

pub trait ProtoHelper {
    type GamePacketType: GamePacketsAll + Send;
}
