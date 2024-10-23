use bedrockrs_proto_core::{GamePacket, GamePacketsAll};

pub trait ProtoHelper {
    type GamePacketType: GamePacketsAll;

}