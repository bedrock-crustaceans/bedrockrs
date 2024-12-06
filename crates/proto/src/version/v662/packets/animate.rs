use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::ActorRuntimeID;

#[derive(ProtoCodec)]
#[enum_repr(i32)]
#[enum_endianness(var)]
#[repr(i32)]
enum Action {
  NoAction = 0,
  Swing = 1,
  WakeUp = 3,
  CriticalHit = 4,
  MagicCriticalHit = 5,
  RowRight {
    #[endianness(le)]
    rowing_time: f32,
  } = 128,
  RowLeft {
    #[endianness(le)]
    rowing_time: f32,
  } = 129,
}

#[gamepacket(id = 44)]
#[derive(ProtoCodec)]
pub struct AnimatePacket {
  pub action: Action,
  pub target_runtime_id: ActorRuntimeID,
}

// TODO: custom proto impl, enum variant