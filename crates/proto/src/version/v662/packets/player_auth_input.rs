use crate::version::v662::enums::{
    ClientPlayMode, InputMode, ItemStackRequestActionType, NewInteractionModel,
    TextProcessingEventOrigin,
};
use crate::version::v662::types::{
    ActorUniqueID, ItemStackRequestSlotInfo, PackedItemUseLegacyInventoryTransaction,
    PlayerBlockActions,
};
use bedrockrs_core::{Vec2, Vec3};
use bedrockrs_macros::{gamepacket, ProtoCodec};
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::{ProtoCodec, ProtoCodecLE, ProtoCodecVAR};
use std::io::Cursor;

#[repr(u64)]
pub enum PlayerAuthInputFlags {
    Ascend = 1 << 0,
    Descend = 1 << 1,
    #[deprecated]
    NorthJump = 1 << 2,
    JumpDown = 1 << 3,
    SprintDown = 1 << 4,
    ChangeHeight = 1 << 5,
    Jumping = 1 << 6,
    AutoJumpingInWater = 1 << 7,
    Sneaking = 1 << 8,
    SneakDown = 1 << 9,
    Up = 1 << 10,
    Down = 1 << 11,
    Left = 1 << 12,
    Right = 1 << 13,
    UpLeft = 1 << 14,
    UpRight = 1 << 15,
    WantUp = 1 << 16,
    WantDown = 1 << 17,
    WantDownSlow = 1 << 18,
    WantUpSlow = 1 << 19,
    Sprinting = 1 << 20,
    AscendBlock = 1 << 21,
    DescendBlock = 1 << 22,
    SneakToggleDown = 1 << 23,
    PersistSneak = 1 << 24,
    StartSprinting = 1 << 25,
    StopSprinting = 1 << 26,
    StartSneaking = 1 << 27,
    StopSneaking = 1 << 28,
    StartSwimming = 1 << 29,
    StopSwimming = 1 << 30,
    StartJumping = 1 << 31,
    StartGliding = 1 << 32,
    StopGliding = 1 << 33,
    PerformItemInteraction = 1 << 34,
    PerformBlockActions = 1 << 35,
    PerformItemStackRequest = 1 << 36,
    HandleTeleport = 1 << 37,
    Emoting = 1 << 38,
    MissedSwing = 1 << 39,
    StartCrawling = 1 << 40,
    StopCrawling = 1 << 41,
    StartFlying = 1 << 42,
    StopFlying = 1 << 43,
    ReceivedServerData = 1 << 44,
    IsInClientPredictedVehicle = 1 << 45,
    PaddleLeft = 1 << 46,
    PaddleRight = 1 << 47,
}

#[derive(ProtoCodec, Clone, Debug)]
struct ActionsEntry {
    pub action_type: ItemStackRequestActionType,
    pub amount: i8,
    pub source: ItemStackRequestSlotInfo,
    pub destination: ItemStackRequestSlotInfo,
}

#[derive(ProtoCodec, Clone, Debug)]
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

#[derive(ProtoCodec, Clone, Debug)]
struct ClientPredictedVehicleData {
    #[endianness(le)]
    pub vehicle_rotation: Vec2<f32>,
    pub client_predicted_vehicle: ActorUniqueID,
}

#[gamepacket(id = 144)]
pub struct PlayerAuthInputPacket {
    pub player_rotation: Vec2<f32>,
    pub player_position: Vec3<f32>,
    pub move_vector: Vec3<f32>,
    pub player_head_rotation: f32,
    pub input_data: u64,
    pub input_mode: InputMode,
    pub play_mode: ClientPlayMode,
    pub new_interaction_model: NewInteractionModel,
    pub vr_gaze_direction: Option<Vec3<f32>>, // If play_mode == ClientPlayMode::Reality
    pub client_tick: u64,
    pub velocity: Vec3<f32>,
    pub item_use_transaction: Option<PackedItemUseLegacyInventoryTransaction>, // If input_data has PlayerAuthInputPacket::InputData::PerformItemInteraction set.
    pub item_stack_request: Option<PerformItemStackRequestData>, // If input data has PlayerAuthInputPacket::InputData::PerformItemStackRequest set.
    pub player_block_actions: Option<PlayerBlockActions>, // If input data has PlayerAuthInputPacket::InputData::PerformBlockActions set.
    pub client_predicted_vehicle: Option<ClientPredictedVehicleData>, // If input data has PlayerAuthInputPacket::InputData::IsInClientPredictedVehicle set.
    pub analog_move_vector: Vec2<f32>,
}

impl ProtoCodec for PlayerAuthInputPacket {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        <Vec2<f32> as ProtoCodecLE>::proto_serialize(&self.player_rotation, stream)?;
        <Vec3<f32> as ProtoCodecLE>::proto_serialize(&self.player_position, stream)?;
        <Vec3<f32> as ProtoCodecLE>::proto_serialize(&self.move_vector, stream)?;
        <f32 as ProtoCodecLE>::proto_serialize(&self.player_head_rotation, stream)?;
        <u64 as ProtoCodecVAR>::proto_serialize(&self.input_data, stream)?;
        <InputMode as ProtoCodec>::proto_serialize(&self.input_mode, stream)?;
        <ClientPlayMode as ProtoCodec>::proto_serialize(&self.play_mode, stream)?;
        <NewInteractionModel as ProtoCodec>::proto_serialize(&self.new_interaction_model, stream)?;
        match &self.play_mode {
            ClientPlayMode::Reality => {
                <Vec3<f32> as ProtoCodecLE>::proto_serialize(
                    &self.vr_gaze_direction.as_ref().unwrap(),
                    stream,
                )?;
            }
            _ => {}
        }
        <u64 as ProtoCodecVAR>::proto_serialize(&self.client_tick, stream)?;
        <Vec3<f32> as ProtoCodecLE>::proto_serialize(&self.velocity, stream)?;
        if &self.input_data & PlayerAuthInputFlags::PerformItemInteraction as u64 != 0 {
            <PackedItemUseLegacyInventoryTransaction as ProtoCodec>::proto_serialize(
                &self.item_use_transaction.as_ref().unwrap(),
                stream,
            )?;
        }
        if &self.input_data & PlayerAuthInputFlags::PerformItemStackRequest as u64 != 0 {
            <PerformItemStackRequestData as ProtoCodec>::proto_serialize(
                &self.item_stack_request.as_ref().unwrap(),
                stream,
            )?;
        }
        if &self.input_data & PlayerAuthInputFlags::PerformBlockActions as u64 != 0 {
            <PlayerBlockActions as ProtoCodec>::proto_serialize(
                &self.player_block_actions.as_ref().unwrap(),
                stream,
            )?;
        }
        if &self.input_data & PlayerAuthInputFlags::IsInClientPredictedVehicle as u64 != 0 {
            <ClientPredictedVehicleData as ProtoCodec>::proto_serialize(
                &self.client_predicted_vehicle.as_ref().unwrap(),
                stream,
            )?;
        }
        <Vec2<f32> as ProtoCodecLE>::proto_serialize(&self.analog_move_vector, stream)?;

        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        let player_rotation = <Vec2<f32> as ProtoCodecLE>::proto_deserialize(stream)?;
        let player_position = <Vec3<f32> as ProtoCodecLE>::proto_deserialize(stream)?;
        let move_vector = <Vec3<f32> as ProtoCodecLE>::proto_deserialize(stream)?;
        let player_head_rotation = <f32 as ProtoCodecLE>::proto_deserialize(stream)?;
        let input_data = <u64 as ProtoCodecVAR>::proto_deserialize(stream)?;
        let input_mode = <InputMode as ProtoCodec>::proto_deserialize(stream)?;
        let play_mode = <ClientPlayMode as ProtoCodec>::proto_deserialize(stream)?;
        let new_interaction_model = <NewInteractionModel as ProtoCodec>::proto_deserialize(stream)?;
        let vr_gaze_direction = match &play_mode {
            ClientPlayMode::Reality => {
                Some(<Vec3<f32> as ProtoCodecLE>::proto_deserialize(stream)?)
            }
            _ => None,
        };
        let client_tick = <u64 as ProtoCodecVAR>::proto_deserialize(stream)?;
        let velocity = <Vec3<f32> as ProtoCodecLE>::proto_deserialize(stream)?;
        let item_use_transaction = match &input_data
            & PlayerAuthInputFlags::PerformItemInteraction as u64
            != 0
        {
            true => Some(
                <PackedItemUseLegacyInventoryTransaction as ProtoCodec>::proto_deserialize(stream)?,
            ),
            false => None,
        };
        let item_stack_request = match &input_data
            & PlayerAuthInputFlags::PerformItemStackRequest as u64
            != 0
        {
            true => Some(<PerformItemStackRequestData as ProtoCodec>::proto_deserialize(stream)?),
            false => None,
        };
        let player_block_actions =
            match &input_data & PlayerAuthInputFlags::PerformBlockActions as u64 != 0 {
                true => Some(<PlayerBlockActions as ProtoCodec>::proto_deserialize(
                    stream,
                )?),
                false => None,
            };
        let client_predicted_vehicle = match &input_data
            & PlayerAuthInputFlags::IsInClientPredictedVehicle as u64
            != 0
        {
            true => Some(<ClientPredictedVehicleData as ProtoCodec>::proto_deserialize(stream)?),
            false => None,
        };
        let analog_move_vector = <Vec2<f32> as ProtoCodecLE>::proto_deserialize(stream)?;

        Ok(Self {
            player_rotation,
            player_position,
            move_vector,
            player_head_rotation,
            input_data,
            input_mode,
            play_mode,
            new_interaction_model,
            vr_gaze_direction,
            client_tick,
            velocity,
            item_use_transaction,
            item_stack_request,
            player_block_actions,
            client_predicted_vehicle,
            analog_move_vector,
        })
    }

    fn get_size_prediction(&self) -> usize {
        ProtoCodecLE::get_size_prediction(&self.player_rotation)
            + ProtoCodecLE::get_size_prediction(&self.player_position)
            + ProtoCodecLE::get_size_prediction(&self.move_vector)
            + ProtoCodecLE::get_size_prediction(&self.player_head_rotation)
            + ProtoCodecVAR::get_size_prediction(&self.input_data)
            + self.input_mode.get_size_prediction()
            + self.play_mode.get_size_prediction()
            + self.new_interaction_model.get_size_prediction()
            + match self.play_mode {
                ClientPlayMode::Reality => ProtoCodecLE::get_size_prediction(&self.vr_gaze_direction),
                _ => 0,
            }
            + ProtoCodecVAR::get_size_prediction(&self.client_tick)
            + ProtoCodecLE::get_size_prediction(&self.velocity)
            + match &self.input_data & PlayerAuthInputFlags::PerformItemInteraction as u64 != 0 {
                true => self.item_use_transaction.get_size_prediction(),
                false => 0,
            }
            + match &self.input_data & PlayerAuthInputFlags::PerformItemStackRequest as u64 != 0 {
                true => self.item_stack_request.get_size_prediction(),
                false => 0,
            }
            + match &self.input_data & PlayerAuthInputFlags::PerformBlockActions as u64 != 0 {
                true => self.player_block_actions.get_size_prediction(),
                false => 0,
            }
            + match &self.input_data & PlayerAuthInputFlags::IsInClientPredictedVehicle as u64
                != 0
            {
                true => self.client_predicted_vehicle.get_size_prediction(),
                false => 0,
            }
            + ProtoCodecLE::get_size_prediction(&self.analog_move_vector)
    }
}

// VERIFY: ProtoCodec impl
