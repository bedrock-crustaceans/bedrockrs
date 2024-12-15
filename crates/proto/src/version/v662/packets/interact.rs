use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::ActorRuntimeID;

#[derive(ProtoCodec)]
#[enum_repr(i8)]
#[repr(i8)]
enum Action {
    Invalid = 0,
    StopRiding {
        #[endianness(le)]
        position_x: f32,
        #[endianness(le)]
        position_y: f32,
        #[endianness(le)]
        position_z: f32,
    } = 3,
    InteractUpdate {
        #[endianness(le)]
        position_x: f32,
        #[endianness(le)]
        position_y: f32,
        #[endianness(le)]
        position_z: f32,
    } = 4,
    NpcOpen = 5,
    OpenInventory = 6,
}

#[gamepacket(id = 33)]
#[derive(ProtoCodec)]
pub struct InteractPacket {
    pub action: Action,
    pub target_runtime_id: ActorRuntimeID,
}

// TODO: custom proto impl because of enum variant serialization order