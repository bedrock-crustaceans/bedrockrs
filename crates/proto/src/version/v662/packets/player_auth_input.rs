use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::{ClientPlayMode, InputMode, ItemStackRequestActionType, NewInteractionModel, TextProcessingEventOrigin};
use crate::version::v662::types::{ActorUniqueID, ItemStackRequestSlotInfo, PackedItemUseLegacyInventoryTransaction, PlayerBlockActions, Vec2, Vec3};

#[derive(ProtoCodec)]
struct ActionsEntry {
    pub action_type: ItemStackRequestActionType,
    pub amount: i8,
    pub source: ItemStackRequestSlotInfo,
    pub destination: ItemStackRequestSlotInfo,
}

#[derive(ProtoCodec)]
struct PerformItemStackRequestData {
    #[endianness(var)]
    pub client_request_id: u32,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub actions: Vec<ActionsEntry>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub strings_to_filter: Vec<String>,
    pub strings_to_filter_origin: TextProcessingEventOrigin,
}

#[derive(ProtoCodec)]
struct ClientPredictedVehicleData {
    pub vehicle_rotation: Vec2,
    pub client_predicted_vehicle: ActorUniqueID,
}

#[gamepacket(id = 144)]
#[derive(ProtoCodec)]
pub struct PlayerAuthInputPacket {
    pub player_rotation: Vec2,
    pub player_position: Vec3,
    pub move_vector: Vec3,
    #[endianness(le)]
    pub player_head_rotation: f32,
    #[endianness(var)]
    pub input_data: u64,
    pub input_mode: InputMode,
    pub play_mode: ClientPlayMode,
    pub new_interaction_model: NewInteractionModel,
    pub vr_gaze_direction: Option<Vec3>, // TODO: custom proto impl, this is added if play_mode == ClientPlayMode::Reality
    #[endianness(var)]
    pub client_tick: u64,
    pub velocity: Vec3,
    pub item_use_transaction: Option<PackedItemUseLegacyInventoryTransaction>, // TODO: custom proto, added if input_data has PlayerAuthInputPacket::InputData::PerformItemInteraction set.
    pub item_stack_request: Option<PerformItemStackRequestData>, // TODO: custom proto, added if input data has PlayerAuthInputPacket::InputData::PerformItemStackRequest set.
    pub player_block_actions: Option<PlayerBlockActions>, // TODO: custom proto, added if input data has PlayerAuthInputPacket::InputData::PerformBlockActions set.
    pub client_predicted_vehicle: Option<ClientPredictedVehicleData>, // TODO: custom proto, added if input data has PlayerAuthInputPacket::InputData::IsInClientPredictedVehicle set.
    pub analog_move_vector: Vec2,
}