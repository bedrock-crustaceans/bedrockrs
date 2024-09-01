use crate::types::input_data::InputData;
use crate::types::input_mode::InputMode;
use crate::types::interaction_model::InteractionModel;
use crate::types::play_mode::PlayMode;
use bedrockrs_core::int::{LE, VAR};
use bedrockrs_core::{Vec2, Vec3};
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;
use bedrockrs_proto_macros::gamepacket;
use bedrockrs_shared::actor_unique_id::ActorUniqueID;
use std::io::Cursor;

#[gamepacket(id = 144)]
#[derive(Debug, Clone)]
pub struct PlayerAuthInputPacket {
    pub rotation: Vec2<LE<f32>>,
    pub position: Vec3<LE<f32>>,
    pub move_vec: Vec2<LE<f32>>,
    pub head_rotation: LE<f32>,
    pub input_data: InputData,
    pub input_mode: InputMode,
    pub play_mode: PlayMode,
    pub interaction_model: InteractionModel,
    /// Which simulation frame client is on. Used to match corrections
    pub client_tick: VAR<u64>,
    /// Velocity
    pub pos_delta: Vec3<LE<f32>>,
    pub analog_move_vec: Vec2<LE<f32>>,
}

macro_rules! set_bit {
    ($v:expr, $bit:expr) => {
        $v |= 1 << $bit
    };
}

macro_rules! get_bit {
    ($v:expr, $bit:expr) => {
        ($v >> $bit) & 1 != 0
    };
}

impl ProtoCodec for PlayerAuthInputPacket {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        todo!()
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        let rotation = Vec2::<LE<f32>>::proto_deserialize(stream)?;
        let position = Vec3::<LE<f32>>::proto_deserialize(stream)?;
        let move_vec = Vec2::<LE<f32>>::proto_deserialize(stream)?;
        let head_rotation = LE::<f32>::proto_deserialize(stream)?;

        let input_data = VAR::<u64>::proto_deserialize(stream)?.into_inner();
        let input_mode = InputMode::proto_deserialize(stream)?;
        let play_mode_int = VAR::<u32>::proto_deserialize(stream)?.into_inner();
        let interaction_model = InteractionModel::proto_deserialize(stream)?;

        let play_mode = match play_mode_int {
            0 => PlayMode::Normal,
            1 => PlayMode::Teaser,
            2 => PlayMode::Screen,
            3 => PlayMode::Viewer,
            4 => {
                let vr_gaze_direction = ProtoCodec::proto_deserialize(stream)?;
                PlayMode::Reality(vr_gaze_direction)
            }
            5 => PlayMode::Placement,
            6 => PlayMode::LivingRoom,
            7 => PlayMode::ExitLevel,
            8 => PlayMode::ExitLevelLivingRoom,
            other => {
                return Err(ProtoCodecError::InvalidEnumID(
                    other.to_string(),
                    String::from("PlayMode"),
                ))
            }
        };

        let client_tick = VAR::<u64>::proto_deserialize(stream)?;
        let pos_delta = Vec3::<LE<f32>>::proto_deserialize(stream)?;

        let input_data = InputData {
            ascend: get_bit!(input_data, 0),
            descend: get_bit!(input_data, 1),
            north_jump_DEPRECATED: get_bit!(input_data, 2),
            jump_down: get_bit!(input_data, 3),
            sprint_down: get_bit!(input_data, 4),
            change_height: get_bit!(input_data, 5),
            jumping: get_bit!(input_data, 6),
            auto_jumping_in_water: get_bit!(input_data, 7),
            sneaking: get_bit!(input_data, 8),
            sneak_down: get_bit!(input_data, 9),
            up: get_bit!(input_data, 10),
            down: get_bit!(input_data, 11),
            left: get_bit!(input_data, 12),
            right: get_bit!(input_data, 13),
            up_left: get_bit!(input_data, 14),
            up_right: get_bit!(input_data, 15),
            want_up: get_bit!(input_data, 16),
            want_down: get_bit!(input_data, 17),
            want_down_slow: get_bit!(input_data, 18),
            want_up_slow: get_bit!(input_data, 19),
            sprinting: get_bit!(input_data, 20),
            ascend_block: get_bit!(input_data, 21),
            descend_block: get_bit!(input_data, 22),
            sneak_toggle_down: get_bit!(input_data, 23),
            persist_sneak: get_bit!(input_data, 24),
            start_sprinting: get_bit!(input_data, 25),
            stop_sprinting: get_bit!(input_data, 26),
            start_sneaking: get_bit!(input_data, 27),
            stop_sneaking: get_bit!(input_data, 28),
            start_swimming: get_bit!(input_data, 29),
            stop_swimming: get_bit!(input_data, 30),
            start_jumping: get_bit!(input_data, 31),
            start_gliding: get_bit!(input_data, 32),
            stop_gliding: get_bit!(input_data, 33),
            perform_item_interaction: get_bit!(input_data, 34),
            perform_block_actions: get_bit!(input_data, 35),
            perform_item_stack_request: get_bit!(input_data, 36),
            handled_teleport: get_bit!(input_data, 37),
            emoting: get_bit!(input_data, 38),
            missed_swing: get_bit!(input_data, 39),
            start_crawling: get_bit!(input_data, 40),
            stop_crawling: get_bit!(input_data, 41),
            start_flying: get_bit!(input_data, 42),
            stop_flying: get_bit!(input_data, 43),
            client_ack_server_data: get_bit!(input_data, 44),
            is_in_client_predicted_vehicle: {
                if get_bit!(input_data, 45) {
                    let vehicle_rotation = Vec2::<LE<f32>>::proto_deserialize(stream)?;
                    let client_predicted_vehicle = ActorUniqueID::proto_deserialize(stream)?;

                    Some((vehicle_rotation, client_predicted_vehicle))
                } else {
                    None
                }
            },
            paddling_left: get_bit!(input_data, 46),
            paddling_right: get_bit!(input_data, 47),
            block_breaking_delay_enabled: get_bit!(input_data, 48),
            input_num: get_bit!(input_data, 49),
        };

        let analog_move_vec = Vec2::<LE<f32>>::proto_deserialize(stream)?;

        Ok(Self {
            rotation,
            position,
            move_vec,
            head_rotation,
            input_data,
            input_mode,
            play_mode,
            interaction_model,
            client_tick,
            pos_delta,
            analog_move_vec,
        })
    }
}
