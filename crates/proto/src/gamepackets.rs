#![allow(non_upper_case_globals)]

use std::io::{Cursor, Write};
use std::sync::Arc;

use crate::packets::add_actor::AddActorPacket;
use crate::packets::add_painting::AddPaintingPacket;
use crate::packets::animate_player::AnimatePlayerPacket;
use crate::packets::camera::CameraPacket;
use crate::packets::chunk_radius_updated::ChunkRadiusUpdatedPacket;
use crate::packets::client_cache_status::ClientCacheStatusPacket;
use crate::packets::command_request::CommandRequestPacket;
use crate::packets::container_close::ContainerClosePacket;
use crate::packets::container_open::ContainerOpenPacket;
use crate::packets::correct_player_move_prediction::CorrectPlayerMovePredictionPacket;
use crate::packets::debug_info::DebugInfoPacket;
use crate::packets::player_disconnect::DisconnectPlayerPacket;
use crate::packets::emote_list::EmoteListPacket;
use crate::packets::handshake_server_to_client::HandshakeServerToClientPacket;
use crate::packets::interact::InteractPacket;
use crate::packets::inventory_content::InventoryContentPacket;
use crate::packets::level_chunk::LevelChunkPacket;
use crate::packets::login::LoginPacket;
use crate::packets::mob_equipment::MobEquipmentPacket;
use crate::packets::modal_form_request::ModalFormRequestPacket;
use crate::packets::modal_form_response::ModalFormResponsePacket;
use crate::packets::network_settings::NetworkSettingsPacket;
use crate::packets::network_settings_request::NetworkSettingsRequestPacket;
use crate::packets::packet_violation_warning::PacketViolationWarningPacket;
use crate::packets::play_status::PlayStatusPacket;
use crate::packets::player_action::PlayerActionPacket;
use crate::packets::player_auth_input::PlayerAuthInputPacket;
use crate::packets::player_hotbar::PlayerHotbarPacket;
use crate::packets::player_move::MovePlayerPacket;
use crate::packets::remove_actor::RemoveEntityPacket;
use crate::packets::chunk_radius_request::ChunkRadiusRequestPacket;
use crate::packets::resource_packs_info::ResourcePacksInfoPacket;
use crate::packets::resource_packs_response::ResourcePacksResponsePacket;
use crate::packets::resource_packs_stack::ResourcePacksStackPacket;
use crate::packets::server_player_post_move_position::ServerPlayerPostMovePositionPacket;
use crate::packets::server_settings_request::ServerSettingsRequestPacket;
use crate::packets::server_settings_response::ServerSettingsResponsePacket;
use crate::packets::set_local_player_as_initialized::SetLocalPlayerAsInitializedPacket;
use crate::packets::set_time::SetTimePacket;
use crate::packets::set_title::SetTitlePacket;
use crate::packets::start_game::StartGamePacket;
use crate::packets::text_message::TextMessagePacket;
use crate::packets::toast_request::ToastRequestPacket;
use bedrockrs_core::int::VAR;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;
use bedrockrs_proto_derive::gamepackets;
use crate::packets::add_player::AddPlayerPacket;
use crate::packets::boss_event::BossEventPacket;
use crate::packets::player_transfer::TransferPacket;
use crate::packets::respawn::RespawnPacket;
use crate::packets::show_credits::ShowCreditsPacket;
use crate::packets::show_profile::ShowProfilePacket;

gamepackets! {
    Login: LoginPacket,
    PlayStatus: PlayStatusPacket,
    ServerToClientHandshake: HandshakeServerToClientPacket,
    ClientToServerHandshake: _,
    DisconnectPlayer: DisconnectPlayerPacket,
    ResourcePacksInfo: ResourcePacksInfoPacket,
    ResourcePackStack: ResourcePacksStackPacket,
    ResourcePackClientResponse: ResourcePacksResponsePacket,
    TextMessage: TextMessagePacket,
    SetTime: SetTimePacket,
    StartGame: StartGamePacket,
    AddPlayer: AddPlayerPacket,
    AddEntity: AddActorPacket,
    RemoveEntity: RemoveEntityPacket,
    AddItemEntity: _,
    ServerPlayerPostMovePosition: ServerPlayerPostMovePositionPacket,
    TakeItemEntity: _,
    MoveEntity: _,
    MovePlayer: MovePlayerPacket,
    RiderJump: _,
    UpdateBlock: _,
    AddPainting: AddPaintingPacket,
    TickSync: _,
    LevelSoundEventV1: _,
    LevelEvent: _,
    BlockEvent: _,
    EntityEvent: _,
    MobEffect: _,
    UpdateAttributes: _,
    InventoryTransaction: _,
    MobEquipment: MobEquipmentPacket,
    MobArmorEquipment: _,
    Interact: InteractPacket,
    BlockPickRequest: _,
    EntityPickRequest: _,
    PlayerAction: PlayerActionPacket,
    HurtArmor: _,
    SetEntityData: _,
    SetEntityMotion: _,
    SetEntityLink: _,
    SetHealth: _,
    SetSpawnPosition: _,
    Animate: AnimatePlayerPacket,
    Respawn: RespawnPacket,
    ContainerOpen: ContainerOpenPacket,
    ContainerClose: ContainerClosePacket,
    PlayerHotbar: PlayerHotbarPacket,
    InventoryContent: InventoryContentPacket,
    InventorySlot: _,
    ContainerSetData: _,
    CraftingData: _,
    GuiDataPickItem: _,
    PlayerInput: _,
    LevelChunk: LevelChunkPacket,
    SetCommandsEnabled: _,
    SetDifficulty: _,
    ChangeDimension: _,
    SetPlayerGameType: _,
    PlayerList: _,
    SimpleEvent: _,
    TelemetryEvent: _,
    SpawnExperienceOrb: _,
    ClientboundMapItemData: _,
    MapInfoRequest: _,
    RequestChunkRadius: ChunkRadiusRequestPacket,
    ChunkRadiusUpdate: ChunkRadiusUpdatedPacket,
    GameRulesChanged: _,
    Camera: CameraPacket,
    BossEvent: BossEventPacket,
    ShowCredits: ShowCreditsPacket,
    AvailableCommands: _,
    CommandRequest: CommandRequestPacket,
    CommandBlockUpdate: _,
    CommandOutput: _,
    UpdateTrade: _,
    UpdateEquipment: _,
    ResourcePackDataInfo: _,
    ResourcePackChunkData: _,
    ResourcePackChunkRequest: _,
    Transfer: TransferPacket,
    PlaySound: _,
    StopSound: _,
    SetTitle: SetTitlePacket,
    AddBehaviorTree: _,
    StructureBlockUpdate: _,
    ShowStoreOffer: _,
    PurchaseReceipt: _,
    PlayerSkin: _,
    SubClientLogin: _,
    InitiateWebSocketConnection: _,
    SetLastHurtBy: _,
    BookEdit: _,
    NpcRequest: _,
    PhotoTransfer: _,
    ModalFormRequest: ModalFormRequestPacket,
    ModalFormResponse: ModalFormResponsePacket,
    ServerSettingsRequest: ServerSettingsRequestPacket,
    ServerSettingsResponse: ServerSettingsResponsePacket,
    ShowProfile: ShowProfilePacket,
    SetDefaultGameType: _,
    RemoveObjective: _,
    SetDisplayObjective: _,
    SetScore: _,
    LabTable: _,
    UpdateBlockSynced: _,
    MoveEntityDelta: _,
    SetScoreboardIdentity: _,
    SetLocalPlayerAsInitialized: SetLocalPlayerAsInitializedPacket,
    UpdateSoftEnum: _,
    NetworkStackLatency: _,
    ScriptCustomEvent: _,
    SpawnParticleEffect: _,
    AvailableEntityIdentifiers: _,
    LevelSoundEventV2: _,
    NetworkChunkPublisherUpdate: _,
    BiomeDefinitionList: _,
    LevelSoundEvent: _,
    LevelEventGeneric: _,
    LecternUpdate: _,
    VideoStreamConnect: _,
    ClientCacheStatus: ClientCacheStatusPacket,
    OnScreenTextureAnimation: _,
    MapCreateLockedCopy: _,
    StructureTemplateDataExportRequest: _,
    StructureTemplateDataExportResponse: _,
    UpdateBlockProperties: _,
    ClientCacheBlobStatus: _,
    ClientCacheMissResponse: _,
    NetworkSettings: NetworkSettingsPacket,
    PlayerAuthInput: PlayerAuthInputPacket,
    CreativeContent: _,
    PlayerEnchantOptions: _,
    ItemStackRequest: _,
    ItemStackResponse: _,
    UpdatePlayerGameType: _,
    EmoteList: EmoteListPacket,
    DebugInfoPacket: DebugInfoPacket,
    PacketViolationWarning: PacketViolationWarningPacket,
    CorrectPlayerMovePredictionPacket: CorrectPlayerMovePredictionPacket,
    ItemComponent: _,
    FilterTextPacket: _,
    UpdateSubChunkBlocksPacket: _,
    SubChunkPacket: _,
    SubChunkRequestPacket: _,
    DimensionData: _,
    ToastRequestPacket: ToastRequestPacket,
    RequestNetworkSettings: NetworkSettingsRequestPacket,
    AlexEntityAnimation: _,
}

impl GamePackets {
    const LoginID: u16 = 1;
    const PlayStatusID: u16 = 2;
    const ServerToClientHandshakeID: u16 = 3;
    const ClientToServerHandshakeID: u16 = 4;
    const DisconnectID: u16 = 5;
    const ResourcePacksInfoID: u16 = 6;
    const ResourcePacksStackID: u16 = 7;
    const ResourcePacksClientResponseID: u16 = 8;
    const TextMessageID: u16 = 9;
    const SetTimeID: u16 = 10;
    const StartGameID: u16 = 11;
    const AddPlayerID: u16 = 12;
    const AddEntityID: u16 = 13;
    const RemoveEntityID: u16 = 14;
    const AddItemEntityID: u16 = 15;
    const ServerPlayerPostMovePositionPacketID: u16 = 16;
    const TakeItemEntityID: u16 = 17;
    const MoveEntityID: u16 = 18;
    const MovePlayerID: u16 = 19;
    const RiderJumpID: u16 = 20;
    const UpdateBlockID: u16 = 21;
    const AddPaintingID: u16 = 22;
    const TickSyncID: u16 = 23;
    const LevelSoundEventOldID: u16 = 24;
    const LevelEventID: u16 = 25;
    const BlockEventID: u16 = 26;
    const EntityEventID: u16 = 27;
    const MobEffectID: u16 = 28;
    const UpdateAttributesID: u16 = 29;
    const InventoryTransactionID: u16 = 30;
    const MobEquipmentID: u16 = 31;
    const MobArmorEquipmentID: u16 = 32;
    const InteractID: u16 = 33;
    const BlockPickRequestID: u16 = 34;
    const EntityPickRequestID: u16 = 35;
    const PlayerActionID: u16 = 36;
    const HurtArmorID: u16 = 38;
    const SetEntityDataID: u16 = 39;
    const SetEntityMotionID: u16 = 40;
    const SetEntityLinkID: u16 = 41;
    const SetHealthID: u16 = 42;
    const SetSpawnPositionID: u16 = 43;
    const AnimateID: u16 = 44;
    const RespawnID: u16 = 45;
    const ContainerOpenID: u16 = 46;
    const ContainerCloseID: u16 = 47;
    const PlayerHotbarID: u16 = 48;
    const InventoryContentID: u16 = 49;
    const InventorySlotID: u16 = 50;
    const ContainerSetDataID: u16 = 51;
    const CraftingDataID: u16 = 52;
    const CraftingEventID: u16 = 53;
    const GuiDataPickItemID: u16 = 54;
    const AdventureSettingsID: u16 = 55;
    const BlockEntityDataID: u16 = 56;
    const PlayerInputID: u16 = 57;
    const LevelChunkID: u16 = 58;
    const SetCommandsEnabledID: u16 = 59;
    const SetDifficultyID: u16 = 60;
    const ChangeDimensionID: u16 = 61;
    const SetPlayerGameTypeID: u16 = 62;
    const PlayerListID: u16 = 63;
    const SimpleEventID: u16 = 64;
    const TelemetryEventID: u16 = 65;
    const SpawnExperienceOrbID: u16 = 66;
    const ClientboundMapItemDataID: u16 = 67;
    const MapInfoRequestID: u16 = 68;
    const RequestChunkRadiusID: u16 = 69;
    const ChunkRadiusUpdateID: u16 = 70;
    const ItemFrameDropItemID: u16 = 71;
    const GameRulesChangedID: u16 = 72;
    const CameraID: u16 = 73;
    const BossEventID: u16 = 74;
    const ShowCreditsID: u16 = 75;
    const AvailableCommandsID: u16 = 76;
    const CommandRequestID: u16 = 77;
    const CommandBlockUpdateID: u16 = 78;
    const CommandOutputID: u16 = 79;
    const UpdateTradeID: u16 = 80;
    const UpdateEquipmentID: u16 = 81;
    const ResourcePackDataInfoID: u16 = 82;
    const ResourcePackChunkDataID: u16 = 83;
    const ResourcePackChunkRequestID: u16 = 84;
    const TransferID: u16 = 85;
    const PlaySoundID: u16 = 86;
    const StopSoundID: u16 = 87;
    const SetTitleID: u16 = 88;
    const AddBehaviorTreeID: u16 = 89;
    const StructureBlockUpdateID: u16 = 90;
    const ShowStoreOfferID: u16 = 91;
    const PurchaseReceiptID: u16 = 92;
    const PlayerSkinID: u16 = 93;
    const SubClientLoginID: u16 = 94;
    const InitiateWebSocketConnectionID: u16 = 95;
    const SetLastHurtByID: u16 = 96;
    const BookEditID: u16 = 97;
    const NpcRequestID: u16 = 98;
    const PhotoTransferID: u16 = 99;
    const ModalFormRequestID: u16 = 100;
    const ModalFormResponseID: u16 = 101;
    const ServerSettingsRequestID: u16 = 102;
    const ServerSettingsResponseID: u16 = 103;
    const ShowProfileID: u16 = 104;
    const SetDefaultGameTypeID: u16 = 105;
    const RemoveObjectiveID: u16 = 106;
    const SetDisplayObjectiveID: u16 = 107;
    const SetScoreID: u16 = 108;
    const LabTableID: u16 = 109;
    const UpdateBlockSyncedID: u16 = 110;
    const MoveEntityDeltaID: u16 = 111;
    const SetScoreboardIdentityID: u16 = 112;
    const SetLocalPlayerAsInitializedID: u16 = 113;
    const UpdateSoftEnumID: u16 = 114;
    const NetworkStackLatencyID: u16 = 115;
    const ScriptCustomEventID: u16 = 117;
    const SpawnParticleEffectID: u16 = 118;
    const AvailableEntityIdentifiersID: u16 = 119;
    const LevelSoundEventV2ID: u16 = 120;
    const NetworkChunkPublisherUpdateID: u16 = 121;
    const BiomeDefinitionListID: u16 = 122;
    const LevelSoundEventID: u16 = 123;
    const LevelEventGenericID: u16 = 124;
    const LecternUpdateID: u16 = 125;
    const VideoStreamConnectID: u16 = 126;
    const ClientCacheStatusID: u16 = 129;
    const OnScreenTextureAnimationID: u16 = 130;
    const MapCreateLockedCopyID: u16 = 131;
    const StructureTemplateDataExportRequestID: u16 = 132;
    const StructureTemplateDataExportResponseID: u16 = 133;
    const UpdateBlockPropertiesID: u16 = 134;
    const ClientCacheBlobStatusID: u16 = 135;
    const ClientCacheMissResponseID: u16 = 136;
    const NetworkSettingsID: u16 = 143;
    const PlayerAuthInputID: u16 = 144;
    const CreativeContentID: u16 = 145;
    const PlayerEnchantOptionsID: u16 = 146;
    const ItemStackRequestID: u16 = 147;
    const ItemStackResponseID: u16 = 148;
    const UpdatePlayerGameTypeID: u16 = 151;
    const EmoteListID: u16 = 152;
    const DebugInfoPacketID: u16 = 155;
    const PacketViolationWarningID: u16 = 156;
    const CorrectPlayerMovePredictionPacketID: u16 = 161;
    const ItemComponentID: u16 = 162;
    const FilterTextPacketID: u16 = 163;
    const UpdateSubChunkBlocksPacketID: u16 = 172;
    const SubChunkPacketID: u16 = 174;
    const SubChunkRequestPacketID: u16 = 175;
    const DimensionDataID: u16 = 180;
    const ToastRequestPackeID: u16 = 186;
    const RequestNetworkSettingsID: u16 = 193;
    const AlexEntityAnimationID: u16 = 224;
}

macro_rules! ser_packet {
    ($stream:expr, $packet_id:expr, $packet_data:expr) => {{
        let mut pk_stream = vec![];

        // TODO add correct header generation
        // let header = "";

        // Write the PacketID to the packet streamfer
        match VAR::<u16>::write(&VAR::new($packet_id), &mut pk_stream) {
            Ok(_) => {}
            Err(e) => {
                return Err(ProtoCodecError::IOError(Arc::new(e)));
            }
        }

        // Write the packet data to the packet stream
        match $packet_data.proto_serialize(&mut pk_stream) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        // Write buffer length
        // TODO: Handle overflow
        match VAR::<u32>::write(&VAR::new(pk_stream.len() as u32), $stream) {
            Ok(_) => {}
            Err(e) => {
                return Err(ProtoCodecError::IOError(Arc::new(e)));
            }
        }

        // Copy pk stream into stream
        match $stream.write_all(pk_stream.as_slice()) {
            Ok(_) => {}
            Err(e) => {
                return Err(ProtoCodecError::IOError(Arc::new(e)));
            }
        }

        Ok(())
    }};
}

macro_rules! de_packet {
    ($stream:expr, $packet_struct:ty) => {{
        match <$packet_struct>::proto_deserialize($stream) {
            Ok(v) => v,
            Err(e) => return Err(e),
        }
    }};
}

impl GamePackets {
    pub fn pk_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        match self {
            GamePackets::Login(pk) => {
                ser_packet!(stream, GamePackets::LoginID, pk)
            }
            GamePackets::PlayStatus(pk) => {
                ser_packet!(stream, GamePackets::PlayStatusID, pk)
            }
            GamePackets::ServerToClientHandshake(pk) => {
                ser_packet!(stream, GamePackets::ServerToClientHandshakeID, pk)
            }
            GamePackets::ClientToServerHandshake() => {
                unimplemented!()
            }
            GamePackets::DisconnectPlayer(pk) => {
                ser_packet!(stream, GamePackets::DisconnectID, pk)
            }
            GamePackets::ResourcePacksInfo(pk) => {
                ser_packet!(stream, GamePackets::ResourcePacksInfoID, pk)
            }
            GamePackets::ResourcePackStack(pk) => {
                ser_packet!(stream, GamePackets::ResourcePacksStackID, pk)
            }
            GamePackets::ResourcePackClientResponse(pk) => {
                ser_packet!(stream, GamePackets::ResourcePacksClientResponseID, pk)
            }
            GamePackets::TextMessage(pk) => {
                ser_packet!(stream, GamePackets::TextMessageID, pk)
            }
            GamePackets::SetTime(pk) => {
                ser_packet!(stream, GamePackets::SetTimeID, pk)
            }
            GamePackets::StartGame(pk) => {
                ser_packet!(stream, GamePackets::StartGameID, pk)
            }
            GamePackets::AddPlayer() => {
                unimplemented!()
            }
            GamePackets::AddEntity(pk) => {
                ser_packet!(stream, GamePackets::AddEntityID, pk)
            }
            GamePackets::RemoveEntity(pk) => {
                ser_packet!(stream, GamePackets::RemoveEntityID, pk)
            }
            GamePackets::AddItemEntity() => {
                unimplemented!()
            }
            GamePackets::ServerPlayerPostMovePositionPacket(pk) => {
                ser_packet!(
                    stream,
                    GamePackets::ServerPlayerPostMovePositionPacketID,
                    pk
                )
            }
            GamePackets::TakeItemEntity() => {
                unimplemented!()
            }
            GamePackets::MoveEntity() => {
                unimplemented!()
            }
            GamePackets::MovePlayer(pk) => {
                ser_packet!(stream, GamePackets::MovePlayerID, pk)
            }
            GamePackets::RiderJump() => {
                unimplemented!()
            }
            GamePackets::UpdateBlock() => {
                unimplemented!()
            }
            GamePackets::AddPainting(pk) => {
                ser_packet!(stream, GamePackets::AddPaintingID, pk)
            }
            GamePackets::TickSync() => {
                unimplemented!()
            }
            GamePackets::LevelSoundEventOld() => {
                unimplemented!()
            }
            GamePackets::LevelEvent() => {
                unimplemented!()
            }
            GamePackets::BlockEvent() => {
                unimplemented!()
            }
            GamePackets::EntityEvent() => {
                unimplemented!()
            }
            GamePackets::MobEffect() => {
                unimplemented!()
            }
            GamePackets::UpdateAttributes() => {
                unimplemented!()
            }
            GamePackets::InventoryTransaction() => {
                unimplemented!()
            }
            GamePackets::MobEquipment(pk) => {
                ser_packet!(stream, GamePackets::MobEquipmentID, pk)
            }
            GamePackets::MobArmorEquipment() => {
                unimplemented!()
            }
            GamePackets::Interact(pk) => {
                ser_packet!(stream, GamePackets::InteractID, pk)
            }
            GamePackets::BlockPickRequest() => {
                unimplemented!()
            }
            GamePackets::EntityPickRequest() => {
                unimplemented!()
            }
            GamePackets::PlayerAction(pk) => {
                ser_packet!(stream, GamePackets::PlayerActionID, pk)
            }
            GamePackets::HurtArmor() => {
                unimplemented!()
            }
            GamePackets::SetEntityData() => {
                unimplemented!()
            }
            GamePackets::SetEntityMotion() => {
                unimplemented!()
            }
            GamePackets::SetEntityLink() => {
                unimplemented!()
            }
            GamePackets::SetHealth() => {
                unimplemented!()
            }
            GamePackets::SetSpawnPosition() => {
                unimplemented!()
            }
            GamePackets::Animate(pk) => {
                ser_packet!(stream, GamePackets::AnimateID, pk)
            }
            GamePackets::Respawn() => {
                unimplemented!()
            }
            GamePackets::ContainerOpen(pk) => {
                ser_packet!(stream, GamePackets::ContainerOpenID, pk)
            }
            GamePackets::ContainerClose(pk) => {
                ser_packet!(stream, GamePackets::ContainerCloseID, pk)
            }
            GamePackets::PlayerHotbar(pk) => {
                ser_packet!(stream, GamePackets::PlayerHotbarID, pk)
            }
            GamePackets::InventoryContent(pk) => {
                ser_packet!(stream, GamePackets::InventoryContentID, pk)
            }
            GamePackets::InventorySlot() => {
                unimplemented!()
            }
            GamePackets::ContainerSetData() => {
                unimplemented!()
            }
            GamePackets::CraftingData() => {
                unimplemented!()
            }
            GamePackets::CraftingEvent() => {
                unimplemented!()
            }
            GamePackets::GuiDataPickItem() => {
                unimplemented!()
            }
            GamePackets::AdventureSettings() => {
                unimplemented!()
            }
            GamePackets::BlockEntityData() => {
                unimplemented!()
            }
            GamePackets::PlayerInput() => {
                unimplemented!()
            }
            GamePackets::LevelChunk(pk) => {
                ser_packet!(stream, GamePackets::LevelChunkID, pk)
            }
            GamePackets::SetCommandsEnabled() => {
                unimplemented!()
            }
            GamePackets::SetDifficulty() => {
                unimplemented!()
            }
            GamePackets::ChangeDimension() => {
                unimplemented!()
            }
            GamePackets::SetPlayerGameType() => {
                unimplemented!()
            }
            GamePackets::PlayerList() => {
                unimplemented!()
            }
            GamePackets::SimpleEvent() => {
                unimplemented!()
            }
            GamePackets::TelemetryEvent() => {
                unimplemented!()
            }
            GamePackets::SpawnExperienceOrb() => {
                unimplemented!()
            }
            GamePackets::ClientboundMapItemData() => {
                unimplemented!()
            }
            GamePackets::MapInfoRequest() => {
                unimplemented!()
            }
            GamePackets::RequestChunkRadius(pk) => {
                ser_packet!(stream, GamePackets::RequestChunkRadiusID, pk)
            }
            GamePackets::ChunkRadiusUpdate(pk) => {
                ser_packet!(stream, GamePackets::ChunkRadiusUpdateID, pk)
            }
            GamePackets::ItemFrameDropItem() => {
                unimplemented!()
            }
            GamePackets::GameRulesChanged() => {
                unimplemented!()
            }
            GamePackets::Camera(pk) => {
                ser_packet!(stream, GamePackets::CameraID, pk)
            }
            GamePackets::BossEvent() => {
                unimplemented!()
            }
            GamePackets::ShowCredits() => {
                unimplemented!()
            }
            GamePackets::AvailableCommands() => {
                unimplemented!()
            }
            GamePackets::CommandRequest(pk) => {
                ser_packet!(stream, GamePackets::CommandRequestID, pk)
            }
            GamePackets::CommandBlockUpdate() => {
                unimplemented!()
            }
            GamePackets::CommandOutput() => {
                unimplemented!()
            }
            GamePackets::UpdateTrade() => {
                unimplemented!()
            }
            GamePackets::UpdateEquipment() => {
                unimplemented!()
            }
            GamePackets::ResourcePackDataInfo() => {
                unimplemented!()
            }
            GamePackets::ResourcePackChunkData() => {
                unimplemented!()
            }
            GamePackets::ResourcePackChunkRequest() => {
                unimplemented!()
            }
            GamePackets::Transfer() => {
                unimplemented!()
            }
            GamePackets::PlaySound() => {
                unimplemented!()
            }
            GamePackets::StopSound() => {
                unimplemented!()
            }
            GamePackets::SetTitle(pk) => {
                ser_packet!(stream, GamePackets::SetTitleID, pk)
            }
            GamePackets::AddBehaviorTree() => {
                unimplemented!()
            }
            GamePackets::StructureBlockUpdate() => {
                unimplemented!()
            }
            GamePackets::ShowStoreOffer() => {
                unimplemented!()
            }
            GamePackets::PurchaseReceipt() => {
                unimplemented!()
            }
            GamePackets::PlayerSkin() => {
                unimplemented!()
            }
            GamePackets::SubClientLogin() => {
                unimplemented!()
            }
            GamePackets::InitiateWebSocketConnection() => {
                unimplemented!()
            }
            GamePackets::SetLastHurtBy() => {
                unimplemented!()
            }
            GamePackets::BookEdit() => {
                unimplemented!()
            }
            GamePackets::NpcRequest() => {
                unimplemented!()
            }
            GamePackets::PhotoTransfer() => {
                unimplemented!()
            }
            GamePackets::ModalFormRequest(pk) => {
                ser_packet!(stream, GamePackets::ModalFormRequestID, pk)
            }
            GamePackets::ModalFormResponse(pk) => {
                ser_packet!(stream, GamePackets::ModalFormResponseID, pk)
            }
            GamePackets::ServerSettingsRequest(pk) => {
                ser_packet!(stream, GamePackets::ServerSettingsRequestID, pk)
            }
            GamePackets::ServerSettingsResponse(pk) => {
                ser_packet!(stream, GamePackets::ServerSettingsResponseID, pk)
            }
            GamePackets::ShowProfile() => {
                unimplemented!()
            }
            GamePackets::SetDefaultGameType() => {
                unimplemented!()
            }
            GamePackets::RemoveObjective() => {
                unimplemented!()
            }
            GamePackets::SetDisplayObjective() => {
                unimplemented!()
            }
            GamePackets::SetScore() => {
                unimplemented!()
            }
            GamePackets::LabTable() => {
                unimplemented!()
            }
            GamePackets::UpdateBlockSynced() => {
                unimplemented!()
            }
            GamePackets::MoveEntityDelta() => {
                unimplemented!()
            }
            GamePackets::SetScoreboardIdentity() => {
                unimplemented!()
            }
            GamePackets::SetLocalPlayerAsInitialized(pk) => {
                ser_packet!(stream, GamePackets::SetLocalPlayerAsInitializedID, pk)
            }
            GamePackets::UpdateSoftEnum() => {
                unimplemented!()
            }
            GamePackets::NetworkStackLatency() => {
                unimplemented!()
            }
            GamePackets::ScriptCustomEvent() => {
                unimplemented!()
            }
            GamePackets::SpawnParticleEffect() => {
                unimplemented!()
            }
            GamePackets::AvailableEntityIdentifiers() => {
                unimplemented!()
            }
            GamePackets::LevelSoundEventV2() => {
                unimplemented!()
            }
            GamePackets::NetworkChunkPublisherUpdate() => {
                unimplemented!()
            }
            GamePackets::BiomeDefinitionList() => {
                unimplemented!()
            }
            GamePackets::LevelSoundEvent() => {
                unimplemented!()
            }
            GamePackets::LevelEventGeneric() => {
                unimplemented!()
            }
            GamePackets::LecternUpdate() => {
                unimplemented!()
            }
            GamePackets::VideoStreamConnect() => {
                unimplemented!()
            }
            GamePackets::ClientCacheStatus(pk) => {
                ser_packet!(stream, GamePackets::ClientCacheStatusID, pk)
            }
            GamePackets::OnScreenTextureAnimation() => {
                unimplemented!()
            }
            GamePackets::MapCreateLockedCopy() => {
                unimplemented!()
            }
            GamePackets::StructureTemplateDataExportRequest() => {
                unimplemented!()
            }
            GamePackets::StructureTemplateDataExportResponse() => {
                unimplemented!()
            }
            GamePackets::UpdateBlockProperties() => {
                unimplemented!()
            }
            GamePackets::ClientCacheBlobStatus() => {
                unimplemented!()
            }
            GamePackets::ClientCacheMissResponse() => {
                unimplemented!()
            }
            GamePackets::NetworkSettings(pk) => {
                ser_packet!(stream, GamePackets::NetworkSettingsID, pk)
            }
            GamePackets::PlayerAuthInput(pk) => {
                ser_packet!(stream, GamePackets::PlayerAuthInputID, pk)
            }
            GamePackets::CreativeContent() => {
                unimplemented!()
            }
            GamePackets::PlayerEnchantOptions() => {
                unimplemented!()
            }
            GamePackets::ItemStackRequest() => {
                unimplemented!()
            }
            GamePackets::ItemStackResponse() => {
                unimplemented!()
            }
            GamePackets::UpdatePlayerGameType() => {
                unimplemented!()
            }
            GamePackets::EmoteList(pk) => {
                ser_packet!(stream, GamePackets::EmoteListID, pk)
            }
            GamePackets::DebugInfoPacket(pk) => {
                ser_packet!(stream, GamePackets::DebugInfoPacketID, pk)
            }
            GamePackets::PacketViolationWarning(pk) => {
                ser_packet!(stream, GamePackets::PacketViolationWarningID, pk)
            }
            GamePackets::CorrectPlayerMovePredictionPacket(pk) => {
                ser_packet!(stream, GamePackets::CorrectPlayerMovePredictionPacketID, pk)
            }
            GamePackets::ItemComponent() => {
                unimplemented!()
            }
            GamePackets::FilterTextPacket() => {
                unimplemented!()
            }
            GamePackets::UpdateSubChunkBlocksPacket() => {
                unimplemented!()
            }
            GamePackets::SubChunkPacket() => {
                unimplemented!()
            }
            GamePackets::SubChunkRequestPacket() => {
                unimplemented!()
            }
            GamePackets::DimensionData() => {
                unimplemented!()
            }
            GamePackets::ToastRequestPacket(pk) => {
                ser_packet!(stream, GamePackets::ToastRequestPackeID, pk)
            }
            GamePackets::RequestNetworkSettings(pk) => {
                ser_packet!(stream, GamePackets::RequestNetworkSettingsID, pk)
            }
            GamePackets::AlexEntityAnimation() => {
                unimplemented!()
            }
        }
    }

    pub fn pk_deserialize(
        stream: &mut Cursor<&[u8]>,
    ) -> Result<(GamePackets, u8, u8), ProtoCodecError> {
        // Read the game packet length
        // We don't need it, yet?
        // TODO: Use this to possibly async the packet handling
        VAR::<u32>::proto_deserialize(stream)?;

        // Read the game packet header and parse it into an u16
        let game_packet_header: u16 = VAR::<u32>::proto_deserialize(stream)?
            .into_inner()
            .try_into()
            .map_err(ProtoCodecError::FromIntError)?;

        // Get the first 10 bits as the packet id
        // Can never be more than a 16-bit integer due to being 10-bits big
        // Gamepacket IDs through 200-299 are used for spin-offs, they are free to use for custom packets
        let game_packet_id = game_packet_header & 0b0000_0011_1111_1111;

        // Get the next 2 bits as the sub client sender id
        // Can never be more than an 8-bit integer due to being 2 bits big
        let sub_client_sender_id = (game_packet_header & 0b0000_1100_0000_0000) as u8;
        // Get the next 2 bits as the sub client target id
        // Can never be more than an 8-bit integer due to being 2 bits big
        let sub_client_target_id = (game_packet_header & 0b0011_0000_0000_0000) as u8;

        // Match the GamePacket to deserialize the correct packet type
        let game_packet = match game_packet_id {
            GamePackets::LoginID => GamePackets::Login(de_packet!(stream, LoginPacket)),
            GamePackets::PlayStatusID => {
                GamePackets::PlayStatus(de_packet!(stream, PlayStatusPacket))
            }
            GamePackets::ServerToClientHandshakeID => GamePackets::ServerToClientHandshake(
                de_packet!(stream, HandshakeServerToClientPacket),
            ),
            GamePackets::ClientToServerHandshakeID => {
                unimplemented!()
            }
            GamePackets::DisconnectID => {
                GamePackets::DisconnectPlayer(de_packet!(stream, DisconnectPlayerPacket))
            }
            GamePackets::ResourcePacksInfoID => {
                GamePackets::ResourcePacksInfo(de_packet!(stream, ResourcePacksInfoPacket))
            }
            GamePackets::ResourcePacksStackID => {
                GamePackets::ResourcePackStack(de_packet!(stream, ResourcePacksStackPacket))
            }
            GamePackets::ResourcePacksClientResponseID => GamePackets::ResourcePackClientResponse(
                de_packet!(stream, ResourcePacksResponsePacket),
            ),
            GamePackets::TextMessageID => {
                GamePackets::TextMessage(de_packet!(stream, TextMessagePacket))
            }
            GamePackets::SetTimeID => GamePackets::SetTime(de_packet!(stream, SetTimePacket)),
            GamePackets::StartGameID => GamePackets::StartGame(de_packet!(stream, StartGamePacket)),
            GamePackets::AddPlayerID => {
                unimplemented!()
            }
            GamePackets::AddEntityID => GamePackets::AddEntity(de_packet!(stream, AddActorPacket)),
            GamePackets::RemoveEntityID => {
                GamePackets::RemoveEntity(de_packet!(stream, RemoveEntityPacket))
            }
            GamePackets::AddItemEntityID => {
                unimplemented!()
            }
            GamePackets::ServerPlayerPostMovePositionPacketID => {
                GamePackets::ServerPlayerPostMovePositionPacket(de_packet!(
                    stream,
                    ServerPlayerPostMovePositionPacket
                ))
            }
            GamePackets::TakeItemEntityID => {
                unimplemented!()
            }
            GamePackets::MoveEntityID => {
                unimplemented!()
            }
            GamePackets::MovePlayerID => {
                GamePackets::MovePlayer(de_packet!(stream, MovePlayerPacket))
            }
            GamePackets::RiderJumpID => {
                unimplemented!()
            }
            GamePackets::UpdateBlockID => {
                unimplemented!()
            }
            GamePackets::AddPaintingID => {
                GamePackets::AddPainting(de_packet!(stream, AddPaintingPacket))
            }
            GamePackets::TickSyncID => {
                unimplemented!()
            }
            GamePackets::LevelSoundEventOldID => {
                unimplemented!()
            }
            GamePackets::LevelEventID => {
                unimplemented!()
            }
            GamePackets::BlockEventID => {
                unimplemented!()
            }
            GamePackets::EntityEventID => {
                unimplemented!()
            }
            GamePackets::MobEffectID => {
                unimplemented!()
            }
            GamePackets::UpdateAttributesID => {
                unimplemented!()
            }
            GamePackets::InventoryTransactionID => {
                unimplemented!()
            }
            GamePackets::MobEquipmentID => {
                GamePackets::MobEquipment(de_packet!(stream, MobEquipmentPacket))
            }
            GamePackets::MobArmorEquipmentID => {
                unimplemented!()
            }
            GamePackets::InteractID => GamePackets::Interact(de_packet!(stream, InteractPacket)),
            GamePackets::BlockPickRequestID => {
                unimplemented!()
            }
            GamePackets::EntityPickRequestID => {
                unimplemented!()
            }
            GamePackets::PlayerActionID => {
                GamePackets::PlayerAction(de_packet!(stream, PlayerActionPacket))
            }
            GamePackets::HurtArmorID => {
                unimplemented!()
            }
            GamePackets::SetEntityDataID => {
                unimplemented!()
            }
            GamePackets::SetEntityMotionID => {
                unimplemented!()
            }
            GamePackets::SetEntityLinkID => {
                unimplemented!()
            }
            GamePackets::SetHealthID => {
                unimplemented!()
            }
            GamePackets::SetSpawnPositionID => {
                unimplemented!()
            }
            GamePackets::AnimateID => GamePackets::Animate(de_packet!(stream, AnimatePlayerPacket)),
            GamePackets::RespawnID => {
                unimplemented!()
            }
            GamePackets::ContainerOpenID => {
                GamePackets::ContainerOpen(de_packet!(stream, ContainerOpenPacket))
            }
            GamePackets::ContainerCloseID => {
                GamePackets::ContainerClose(de_packet!(stream, ContainerClosePacket))
            }
            GamePackets::PlayerHotbarID => {
                GamePackets::PlayerHotbar(de_packet!(stream, PlayerHotbarPacket))
            }
            GamePackets::InventoryContentID => {
                GamePackets::InventoryContent(de_packet!(stream, InventoryContentPacket))
            }
            GamePackets::InventorySlotID => {
                unimplemented!()
            }
            GamePackets::ContainerSetDataID => {
                unimplemented!()
            }
            GamePackets::CraftingDataID => {
                unimplemented!()
            }
            GamePackets::CraftingEventID => {
                unimplemented!()
            }
            GamePackets::GuiDataPickItemID => {
                unimplemented!()
            }
            GamePackets::AdventureSettingsID => {
                unimplemented!()
            }
            GamePackets::BlockEntityDataID => {
                unimplemented!()
            }
            GamePackets::PlayerInputID => {
                unimplemented!()
            }
            GamePackets::LevelChunkID => {
                unimplemented!()
            }
            GamePackets::SetCommandsEnabledID => {
                unimplemented!()
            }
            GamePackets::SetDifficultyID => {
                unimplemented!()
            }
            GamePackets::ChangeDimensionID => {
                unimplemented!()
            }
            GamePackets::SetPlayerGameTypeID => {
                unimplemented!()
            }
            GamePackets::PlayerListID => {
                unimplemented!()
            }
            GamePackets::SimpleEventID => {
                unimplemented!()
            }
            GamePackets::TelemetryEventID => {
                unimplemented!()
            }
            GamePackets::SpawnExperienceOrbID => {
                unimplemented!()
            }
            GamePackets::ClientboundMapItemDataID => {
                unimplemented!()
            }
            GamePackets::MapInfoRequestID => {
                unimplemented!()
            }
            GamePackets::RequestChunkRadiusID => {
                GamePackets::RequestChunkRadius(de_packet!(stream, ChunkRadiusRequestPacket))
            }
            GamePackets::ChunkRadiusUpdateID => {
                unimplemented!()
            }
            GamePackets::ItemFrameDropItemID => {
                unimplemented!()
            }
            GamePackets::GameRulesChangedID => {
                unimplemented!()
            }
            GamePackets::CameraID => GamePackets::Camera(de_packet!(stream, CameraPacket)),
            GamePackets::BossEventID => {
                unimplemented!()
            }
            GamePackets::ShowCreditsID => {
                unimplemented!()
            }
            GamePackets::AvailableCommandsID => {
                unimplemented!()
            }
            GamePackets::CommandRequestID => {
                GamePackets::CommandRequest(de_packet!(stream, CommandRequestPacket))
            }
            GamePackets::CommandBlockUpdateID => {
                unimplemented!()
            }
            GamePackets::CommandOutputID => {
                unimplemented!()
            }
            GamePackets::UpdateTradeID => {
                unimplemented!()
            }
            GamePackets::UpdateEquipmentID => {
                unimplemented!()
            }
            GamePackets::ResourcePackDataInfoID => {
                unimplemented!()
            }
            GamePackets::ResourcePackChunkDataID => {
                unimplemented!()
            }
            GamePackets::ResourcePackChunkRequestID => {
                unimplemented!()
            }
            GamePackets::TransferID => {
                unimplemented!()
            }
            GamePackets::PlaySoundID => {
                unimplemented!()
            }
            GamePackets::StopSoundID => {
                unimplemented!()
            }
            GamePackets::SetTitleID => GamePackets::SetTitle(de_packet!(stream, SetTitlePacket)),
            GamePackets::AddBehaviorTreeID => {
                unimplemented!()
            }
            GamePackets::StructureBlockUpdateID => {
                unimplemented!()
            }
            GamePackets::ShowStoreOfferID => {
                unimplemented!()
            }
            GamePackets::PurchaseReceiptID => {
                unimplemented!()
            }
            GamePackets::PlayerSkinID => {
                unimplemented!()
            }
            GamePackets::SubClientLoginID => {
                unimplemented!()
            }
            GamePackets::InitiateWebSocketConnectionID => {
                unimplemented!()
            }
            GamePackets::SetLastHurtByID => {
                unimplemented!()
            }
            GamePackets::BookEditID => {
                unimplemented!()
            }
            GamePackets::NpcRequestID => {
                unimplemented!()
            }
            GamePackets::PhotoTransferID => {
                unimplemented!()
            }
            GamePackets::ModalFormRequestID => {
                GamePackets::ModalFormRequest(de_packet!(stream, ModalFormRequestPacket))
            }
            GamePackets::ModalFormResponseID => {
                GamePackets::ModalFormResponse(de_packet!(stream, ModalFormResponsePacket))
            }
            GamePackets::ServerSettingsRequestID => {
                GamePackets::ServerSettingsRequest(de_packet!(stream, ServerSettingsRequestPacket))
            }
            GamePackets::ServerSettingsResponseID => GamePackets::ServerSettingsResponse(
                de_packet!(stream, ServerSettingsResponsePacket),
            ),
            GamePackets::ShowProfileID => {
                unimplemented!()
            }
            GamePackets::SetDefaultGameTypeID => {
                unimplemented!()
            }
            GamePackets::RemoveObjectiveID => {
                unimplemented!()
            }
            GamePackets::SetDisplayObjectiveID => {
                unimplemented!()
            }
            GamePackets::SetScoreID => {
                unimplemented!()
            }
            GamePackets::LabTableID => {
                unimplemented!()
            }
            GamePackets::UpdateBlockSyncedID => {
                unimplemented!()
            }
            GamePackets::MoveEntityDeltaID => {
                unimplemented!()
            }
            GamePackets::SetScoreboardIdentityID => {
                unimplemented!()
            }
            GamePackets::SetLocalPlayerAsInitializedID => GamePackets::SetLocalPlayerAsInitialized(
                de_packet!(stream, SetLocalPlayerAsInitializedPacket),
            ),
            GamePackets::UpdateSoftEnumID => {
                unimplemented!()
            }
            GamePackets::NetworkStackLatencyID => {
                unimplemented!()
            }
            GamePackets::ScriptCustomEventID => {
                unimplemented!()
            }
            GamePackets::SpawnParticleEffectID => {
                unimplemented!()
            }
            GamePackets::AvailableEntityIdentifiersID => {
                unimplemented!()
            }
            GamePackets::LevelSoundEventV2ID => {
                unimplemented!()
            }
            GamePackets::NetworkChunkPublisherUpdateID => {
                unimplemented!()
            }
            GamePackets::BiomeDefinitionListID => {
                unimplemented!()
            }
            GamePackets::LevelSoundEventID => {
                unimplemented!()
            }
            GamePackets::LevelEventGenericID => {
                unimplemented!()
            }
            GamePackets::LecternUpdateID => {
                unimplemented!()
            }
            GamePackets::VideoStreamConnectID => {
                unimplemented!()
            }
            GamePackets::ClientCacheStatusID => {
                GamePackets::ClientCacheStatus(de_packet!(stream, ClientCacheStatusPacket))
            }
            GamePackets::OnScreenTextureAnimationID => {
                unimplemented!()
            }
            GamePackets::MapCreateLockedCopyID => {
                unimplemented!()
            }
            GamePackets::StructureTemplateDataExportRequestID => {
                unimplemented!()
            }
            GamePackets::StructureTemplateDataExportResponseID => {
                unimplemented!()
            }
            GamePackets::UpdateBlockPropertiesID => {
                unimplemented!()
            }
            GamePackets::ClientCacheBlobStatusID => {
                unimplemented!()
            }
            GamePackets::ClientCacheMissResponseID => {
                unimplemented!()
            }
            GamePackets::NetworkSettingsID => {
                GamePackets::NetworkSettings(de_packet!(stream, NetworkSettingsPacket))
            }
            GamePackets::PlayerAuthInputID => {
                GamePackets::PlayerAuthInput(de_packet!(stream, PlayerAuthInputPacket))
            }
            GamePackets::CreativeContentID => {
                unimplemented!()
            }
            GamePackets::PlayerEnchantOptionsID => {
                unimplemented!()
            }
            GamePackets::ItemStackRequestID => {
                unimplemented!()
            }
            GamePackets::ItemStackResponseID => {
                unimplemented!()
            }
            GamePackets::UpdatePlayerGameTypeID => {
                unimplemented!()
            }
            GamePackets::EmoteListID => GamePackets::EmoteList(de_packet!(stream, EmoteListPacket)),
            GamePackets::DebugInfoPacketID => {
                GamePackets::DebugInfoPacket(de_packet!(stream, DebugInfoPacket))
            }
            GamePackets::PacketViolationWarningID => GamePackets::PacketViolationWarning(
                de_packet!(stream, PacketViolationWarningPacket),
            ),
            GamePackets::CorrectPlayerMovePredictionPacketID => {
                GamePackets::CorrectPlayerMovePredictionPacket(de_packet!(
                    stream,
                    CorrectPlayerMovePredictionPacket
                ))
            }
            GamePackets::ItemComponentID => {
                unimplemented!()
            }
            GamePackets::FilterTextPacketID => {
                unimplemented!()
            }
            GamePackets::UpdateSubChunkBlocksPacketID => {
                unimplemented!()
            }
            GamePackets::SubChunkPacketID => {
                unimplemented!()
            }
            GamePackets::SubChunkRequestPacketID => {
                unimplemented!()
            }
            GamePackets::DimensionDataID => {
                unimplemented!()
            }
            GamePackets::ToastRequestPackeID => {
                GamePackets::ToastRequestPacket(de_packet!(stream, ToastRequestPacket))
            }
            GamePackets::RequestNetworkSettingsID => GamePackets::RequestNetworkSettings(
                de_packet!(stream, NetworkSettingsRequestPacket),
            ),
            GamePackets::AlexEntityAnimationID => {
                unimplemented!()
            }
            other => {
                return Err(ProtoCodecError::InvalidGamePacketID(other));
            }
        };

        Ok((game_packet, sub_client_sender_id, sub_client_target_id))
    }
}
