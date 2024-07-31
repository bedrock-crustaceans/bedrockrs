use std::io::Cursor;
use bedrockrs_core::int::{LE, VAR};
use bedrockrs_core::{Vec2, Vec3};
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;
use bedrockrs_proto_derive::ProtoCodec;
use crate::types::input_data::InputData;
use crate::types::input_mode::InputMode;
use crate::types::interaction_model::InteractionModel;
use crate::types::play_mode::PlayMode;

#[derive(ProtoCodec, Debug, Clone)]
pub struct PlayerAuthInputPacket {
    rotation: Vec2<LE<f32>>,
    position: Vec3<LE<f32>>,
    move_vec: Vec2<LE<f32>>,
    head_rotation: LE<f32>,
    input_data: InputData,
    input_mode: InputMode,
    play_mode: PlayMode,
    interaction_model: InteractionModel,
    /// Which simulation frame client is on. Used to match corrections
    client_tick: VAR<u64>,
    /// Velocity
    pos_delta: Vec3<LE<f32>>,
    analog_move_vec: Vec2<LE<f32>>,
}

// TODO: Manual impl
// impl ProtoCodec for PlayerAuthInputPacket {
//     fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
//         todo!()
//     }
//
//     fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
//         todo!()
//     }
// }
