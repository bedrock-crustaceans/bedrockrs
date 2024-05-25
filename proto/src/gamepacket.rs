#![allow(non_upper_case_globals)]

use std::io::{Cursor, Write};

use proto_core::error::ProtoCodecError;
use proto_core::ProtoCodec;
use varint_rs::{VarintReader, VarintWriter};

use crate::packets::client_cache_status::ClientCacheStatusPacket;
use crate::packets::disconnect::DisconnectPacket;
use crate::packets::handshake_server_to_client::HandshakeServerToClientPacket;
use crate::packets::login::LoginPacket;
use crate::packets::network_settings::NetworkSettingsPacket;
use crate::packets::network_settings_request::NetworkSettingsRequestPacket;
use crate::packets::play_status::PlayStatusPacket;
use crate::packets::resource_packs_info::ResourcePacksInfoPacket;
use crate::packets::resource_packs_response::ResourcePacksResponsePacket;
use crate::packets::resource_packs_stack::ResourcePacksStackPacket;

#[repr(u16)]
#[derive(Debug)]
pub enum GamePacket {
    Login(LoginPacket),
    PlayStatus(PlayStatusPacket),
    ServerToClientHandshake(HandshakeServerToClientPacket),
    ClientToServerHandshake(),
    Disconnect(DisconnectPacket),
    ResourcePacksInfo(ResourcePacksInfoPacket),
    ResourcePackStack(ResourcePacksStackPacket),
    ResourcePackClientResponse(ResourcePacksResponsePacket),
    Text(),
    SetTime(),
    StartGame(),
    AddPlayer(),
    AddEntity(),
    RemoveEntity(),
    AddItemEntity(),
    TakeItemEntity(),
    MoveEntity(),
    MovePlayer(),
    RiderJump(),
    UpdateBlock(),
    AddPainting(),
    TickSync(),
    LevelSoundEventOld(),
    LevelEvent(),
    BlockEvent(),
    EntityEvent(),
    MobEffect(),
    UpdateAttributes(),
    InventoryTransaction(),
    MobEquipment(),
    MobArmorEquipment(),
    Interact(),
    BlockPickRequest(),
    EntityPickRequest(),
    PlayerAction(),
    HurtArmor(),
    SetEntityData(),
    SetEntityMotion(),
    SetEntityLink(),
    SetHealth(),
    SetSpawnPosition(),
    Animate(),
    Respawn(),
    ContainerOpen(),
    ContainerClose(),
    PlayerHotbar(),
    InventoryContent(),
    InventorySlot(),
    ContainerSetData(),
    CraftingData(),
    CraftingEvent(),
    GuiDataPickItem(),
    AdventureSettings(),
    BlockEntityData(),
    PlayerInput(),
    LevelChunk(),
    SetCommandsEnabled(),
    SetDifficulty(),
    ChangeDimension(),
    SetPlayerGameType(),
    PlayerList(),
    SimpleEvent(),
    TelemetryEvent(),
    SpawnExperienceOrb(),
    ClientboundMapItemData(),
    MapInfoRequest(),
    RequestChunkRadius(),
    ChunkRadiusUpdate(),
    ItemFrameDropItem(),
    GameRulesChanged(),
    Camera(),
    BossEvent(),
    ShowCredits(),
    AvailableCommands(),
    CommandRequest(),
    CommandBlockUpdate(),
    CommandOutput(),
    UpdateTrade(),
    UpdateEquipment(),
    ResourcePackDataInfo(),
    ResourcePackChunkData(),
    ResourcePackChunkRequest(),
    Transfer(),
    PlaySound(),
    StopSound(),
    SetTitle(),
    AddBehaviorTree(),
    StructureBlockUpdate(),
    ShowStoreOffer(),
    PurchaseReceipt(),
    PlayerSkin(),
    SubClientLogin(),
    InitiateWebSocketConnection(),
    SetLastHurtBy(),
    BookEdit(),
    NpcRequest(),
    PhotoTransfer(),
    ModalFormRequest(),
    ModalFormResponse(),
    ServerSettingsRequest(),
    ServerSettingsResponse(),
    ShowProfile(),
    SetDefaultGameType(),
    RemoveObjective(),
    SetDisplayObjective(),
    SetScore(),
    LabTable(),
    UpdateBlockSynced(),
    MoveEntityDelta(),
    SetScoreboardIdentity(),
    SetLocalPlayerAsInitialized(),
    UpdateSoftEnum(),
    NetworkStackLatency(),
    ScriptCustomEvent(),
    SpawnParticleEffect(),
    AvailableEntityIdentifiers(),
    LevelSoundEventV2(),
    NetworkChunkPublisherUpdate(),
    BiomeDefinitionList(),
    LevelSoundEvent(),
    LevelEventGeneric(),
    LecternUpdate(),
    VideoStreamConnect(),
    ClientCacheStatus(ClientCacheStatusPacket),
    OnScreenTextureAnimation(),
    MapCreateLockedCopy(),
    StructureTemplateDataExportRequest(),
    StructureTemplateDataExportResponse(),
    UpdateBlockProperties(),
    ClientCacheBlobStatus(),
    ClientCacheMissResponse(),
    NetworkSettings(NetworkSettingsPacket),
    PlayerAuthInput(),
    CreativeContent(),
    PlayerEnchantOptions(),
    ItemStackRequest(),
    ItemStackResponse(),
    UpdatePlayerGameType(),
    PacketViolationWarning(),
    ItemComponent(),
    FilterTextPacket(),
    UpdateSubChunkBlocksPacket(),
    SubChunkPacket(),
    SubChunkRequestPacket(),
    DimensionData(),
    RequestNetworkSettings(NetworkSettingsRequestPacket),
    AlexEntityAnimation(),
}

impl GamePacket {
    const LoginID: u16 = 1;
    const PlayStatusID: u16 = 2;
    const ServerToClientHandshakeID: u16 = 3;
    const ClientToServerHandshakeID: u16 = 4;
    const DisconnectID: u16 = 5;
    const ResourcePacksInfoID: u16 = 6;
    const ResourcePacksStackID: u16 = 7;
    const ResourcePacksClientResponseID: u16 = 8;
    const TextID: u16 = 9;
    const SetTimeID: u16 = 10;
    const StartGameID: u16 = 11;
    const AddPlayerID: u16 = 12;
    const AddEntityID: u16 = 13;
    const RemoveEntityID: u16 = 14;
    const AddItemEntityID: u16 = 15;
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
    const PacketViolationWarningID: u16 = 156;
    const ItemComponentID: u16 = 162;
    const FilterTextPacketID: u16 = 163;
    const UpdateSubChunkBlocksPacketID: u16 = 172;
    const SubChunkPacketID: u16 = 174;
    const SubChunkRequestPacketID: u16 = 175;
    const DimensionDataID: u16 = 180;
    const RequestNetworkSettingsID: u16 = 193;
    const AlexEntityAnimationID: u16 = 224;
}

macro_rules! ser_packet {
    ($buf:expr, $packet_id:expr, $packet_data:expr) => {{
        let mut pk_buf = vec![];

        // TODO add correct header generation
        // let header = "";

        // Write the PacketID to the packet buffer
        match pk_buf.write_u16_varint($packet_id as u16) {
            Ok(_) => {}
            Err(e) => {
                return Err(ProtoCodecError::IOError(e));
            }
        }

        // Write the packet data to the packet buffer
        match $packet_data.proto_serialize(&mut pk_buf) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        // Write buffer length
        match $buf.write_u32_varint(pk_buf.len() as u32) {
            Ok(_) => {}
            Err(e) => {
                return Err(ProtoCodecError::IOError(e));
            }
        }

        // Copy pk buffer into buffer
        match $buf.write_all(&*pk_buf) {
            Ok(_) => {}
            Err(e) => {
                return Err(ProtoCodecError::IOError(e));
            }
        }

        Ok(())
    }};
}

macro_rules! de_packet {
    ($cursor:expr, $packet_struct:ty) => {{
        match <$packet_struct>::proto_deserialize($cursor) {
            Ok(v) => v,
            Err(e) => return Err(e),
        }
    }};
}

impl GamePacket {
    pub fn pk_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        match self {
            GamePacket::Login(pk) => {
                ser_packet!(buf, GamePacket::Login as u16, pk)
            }
            GamePacket::PlayStatus(pk) => {
                ser_packet!(buf, GamePacket::PlayStatusID, pk)
            }
            GamePacket::ServerToClientHandshake(pk) => {
                ser_packet!(buf, GamePacket::ServerToClientHandshakeID, pk)
            }
            GamePacket::ClientToServerHandshake() => {
                unimplemented!()
            }
            GamePacket::Disconnect(pk) => {
                ser_packet!(buf, GamePacket::DisconnectID, pk)
            }
            GamePacket::ResourcePacksInfo(pk) => {
                ser_packet!(buf, GamePacket::ResourcePacksInfoID, pk)
            }
            GamePacket::ResourcePackStack(pk) => {
                ser_packet!(buf, GamePacket::ResourcePacksStackID, pk)
            }
            GamePacket::ResourcePackClientResponse(pk) => {
                ser_packet!(buf, GamePacket::ResourcePacksClientResponseID, pk)
            }
            GamePacket::Text() => {
                unimplemented!()
            }
            GamePacket::SetTime() => {
                unimplemented!()
            }
            GamePacket::StartGame() => {
                unimplemented!()
            }
            GamePacket::AddPlayer() => {
                unimplemented!()
            }
            GamePacket::AddEntity() => {
                unimplemented!()
            }
            GamePacket::RemoveEntity() => {
                unimplemented!()
            }
            GamePacket::AddItemEntity() => {
                unimplemented!()
            }
            GamePacket::TakeItemEntity() => {
                unimplemented!()
            }
            GamePacket::MoveEntity() => {
                unimplemented!()
            }
            GamePacket::MovePlayer() => {
                unimplemented!()
            }
            GamePacket::RiderJump() => {
                unimplemented!()
            }
            GamePacket::UpdateBlock() => {
                unimplemented!()
            }
            GamePacket::AddPainting() => {
                unimplemented!()
            }
            GamePacket::TickSync() => {
                unimplemented!()
            }
            GamePacket::LevelSoundEventOld() => {
                unimplemented!()
            }
            GamePacket::LevelEvent() => {
                unimplemented!()
            }
            GamePacket::BlockEvent() => {
                unimplemented!()
            }
            GamePacket::EntityEvent() => {
                unimplemented!()
            }
            GamePacket::MobEffect() => {
                unimplemented!()
            }
            GamePacket::UpdateAttributes() => {
                unimplemented!()
            }
            GamePacket::InventoryTransaction() => {
                unimplemented!()
            }
            GamePacket::MobEquipment() => {
                unimplemented!()
            }
            GamePacket::MobArmorEquipment() => {
                unimplemented!()
            }
            GamePacket::Interact() => {
                unimplemented!()
            }
            GamePacket::BlockPickRequest() => {
                unimplemented!()
            }
            GamePacket::EntityPickRequest() => {
                unimplemented!()
            }
            GamePacket::PlayerAction() => {
                unimplemented!()
            }
            GamePacket::HurtArmor() => {
                unimplemented!()
            }
            GamePacket::SetEntityData() => {
                unimplemented!()
            }
            GamePacket::SetEntityMotion() => {
                unimplemented!()
            }
            GamePacket::SetEntityLink() => {
                unimplemented!()
            }
            GamePacket::SetHealth() => {
                unimplemented!()
            }
            GamePacket::SetSpawnPosition() => {
                unimplemented!()
            }
            GamePacket::Animate() => {
                unimplemented!()
            }
            GamePacket::Respawn() => {
                unimplemented!()
            }
            GamePacket::ContainerOpen() => {
                unimplemented!()
            }
            GamePacket::ContainerClose() => {
                unimplemented!()
            }
            GamePacket::PlayerHotbar() => {
                unimplemented!()
            }
            GamePacket::InventoryContent() => {
                unimplemented!()
            }
            GamePacket::InventorySlot() => {
                unimplemented!()
            }
            GamePacket::ContainerSetData() => {
                unimplemented!()
            }
            GamePacket::CraftingData() => {
                unimplemented!()
            }
            GamePacket::CraftingEvent() => {
                unimplemented!()
            }
            GamePacket::GuiDataPickItem() => {
                unimplemented!()
            }
            GamePacket::AdventureSettings() => {
                unimplemented!()
            }
            GamePacket::BlockEntityData() => {
                unimplemented!()
            }
            GamePacket::PlayerInput() => {
                unimplemented!()
            }
            GamePacket::LevelChunk() => {
                unimplemented!()
            }
            GamePacket::SetCommandsEnabled() => {
                unimplemented!()
            }
            GamePacket::SetDifficulty() => {
                unimplemented!()
            }
            GamePacket::ChangeDimension() => {
                unimplemented!()
            }
            GamePacket::SetPlayerGameType() => {
                unimplemented!()
            }
            GamePacket::PlayerList() => {
                unimplemented!()
            }
            GamePacket::SimpleEvent() => {
                unimplemented!()
            }
            GamePacket::TelemetryEvent() => {
                unimplemented!()
            }
            GamePacket::SpawnExperienceOrb() => {
                unimplemented!()
            }
            GamePacket::ClientboundMapItemData() => {
                unimplemented!()
            }
            GamePacket::MapInfoRequest() => {
                unimplemented!()
            }
            GamePacket::RequestChunkRadius() => {
                unimplemented!()
            }
            GamePacket::ChunkRadiusUpdate() => {
                unimplemented!()
            }
            GamePacket::ItemFrameDropItem() => {
                unimplemented!()
            }
            GamePacket::GameRulesChanged() => {
                unimplemented!()
            }
            GamePacket::Camera() => {
                unimplemented!()
            }
            GamePacket::BossEvent() => {
                unimplemented!()
            }
            GamePacket::ShowCredits() => {
                unimplemented!()
            }
            GamePacket::AvailableCommands() => {
                unimplemented!()
            }
            GamePacket::CommandRequest() => {
                unimplemented!()
            }
            GamePacket::CommandBlockUpdate() => {
                unimplemented!()
            }
            GamePacket::CommandOutput() => {
                unimplemented!()
            }
            GamePacket::UpdateTrade() => {
                unimplemented!()
            }
            GamePacket::UpdateEquipment() => {
                unimplemented!()
            }
            GamePacket::ResourcePackDataInfo() => {
                unimplemented!()
            }
            GamePacket::ResourcePackChunkData() => {
                unimplemented!()
            }
            GamePacket::ResourcePackChunkRequest() => {
                unimplemented!()
            }
            GamePacket::Transfer() => {
                unimplemented!()
            }
            GamePacket::PlaySound() => {
                unimplemented!()
            }
            GamePacket::StopSound() => {
                unimplemented!()
            }
            GamePacket::SetTitle() => {
                unimplemented!()
            }
            GamePacket::AddBehaviorTree() => {
                unimplemented!()
            }
            GamePacket::StructureBlockUpdate() => {
                unimplemented!()
            }
            GamePacket::ShowStoreOffer() => {
                unimplemented!()
            }
            GamePacket::PurchaseReceipt() => {
                unimplemented!()
            }
            GamePacket::PlayerSkin() => {
                unimplemented!()
            }
            GamePacket::SubClientLogin() => {
                unimplemented!()
            }
            GamePacket::InitiateWebSocketConnection() => {
                unimplemented!()
            }
            GamePacket::SetLastHurtBy() => {
                unimplemented!()
            }
            GamePacket::BookEdit() => {
                unimplemented!()
            }
            GamePacket::NpcRequest() => {
                unimplemented!()
            }
            GamePacket::PhotoTransfer() => {
                unimplemented!()
            }
            GamePacket::ModalFormRequest() => {
                unimplemented!()
            }
            GamePacket::ModalFormResponse() => {
                unimplemented!()
            }
            GamePacket::ServerSettingsRequest() => {
                unimplemented!()
            }
            GamePacket::ServerSettingsResponse() => {
                unimplemented!()
            }
            GamePacket::ShowProfile() => {
                unimplemented!()
            }
            GamePacket::SetDefaultGameType() => {
                unimplemented!()
            }
            GamePacket::RemoveObjective() => {
                unimplemented!()
            }
            GamePacket::SetDisplayObjective() => {
                unimplemented!()
            }
            GamePacket::SetScore() => {
                unimplemented!()
            }
            GamePacket::LabTable() => {
                unimplemented!()
            }
            GamePacket::UpdateBlockSynced() => {
                unimplemented!()
            }
            GamePacket::MoveEntityDelta() => {
                unimplemented!()
            }
            GamePacket::SetScoreboardIdentity() => {
                unimplemented!()
            }
            GamePacket::SetLocalPlayerAsInitialized() => {
                unimplemented!()
            }
            GamePacket::UpdateSoftEnum() => {
                unimplemented!()
            }
            GamePacket::NetworkStackLatency() => {
                unimplemented!()
            }
            GamePacket::ScriptCustomEvent() => {
                unimplemented!()
            }
            GamePacket::SpawnParticleEffect() => {
                unimplemented!()
            }
            GamePacket::AvailableEntityIdentifiers() => {
                unimplemented!()
            }
            GamePacket::LevelSoundEventV2() => {
                unimplemented!()
            }
            GamePacket::NetworkChunkPublisherUpdate() => {
                unimplemented!()
            }
            GamePacket::BiomeDefinitionList() => {
                unimplemented!()
            }
            GamePacket::LevelSoundEvent() => {
                unimplemented!()
            }
            GamePacket::LevelEventGeneric() => {
                unimplemented!()
            }
            GamePacket::LecternUpdate() => {
                unimplemented!()
            }
            GamePacket::VideoStreamConnect() => {
                unimplemented!()
            }
            GamePacket::ClientCacheStatus(pk) => {
                ser_packet!(buf, GamePacket::ClientCacheStatusID, pk)
            }
            GamePacket::OnScreenTextureAnimation() => {
                unimplemented!()
            }
            GamePacket::MapCreateLockedCopy() => {
                unimplemented!()
            }
            GamePacket::StructureTemplateDataExportRequest() => {
                unimplemented!()
            }
            GamePacket::StructureTemplateDataExportResponse() => {
                unimplemented!()
            }
            GamePacket::UpdateBlockProperties() => {
                unimplemented!()
            }
            GamePacket::ClientCacheBlobStatus() => {
                unimplemented!()
            }
            GamePacket::ClientCacheMissResponse() => {
                unimplemented!()
            }
            GamePacket::NetworkSettings(pk) => {
                ser_packet!(buf, GamePacket::NetworkSettingsID, pk)
            }
            GamePacket::PlayerAuthInput() => {
                unimplemented!()
            }
            GamePacket::CreativeContent() => {
                unimplemented!()
            }
            GamePacket::PlayerEnchantOptions() => {
                unimplemented!()
            }
            GamePacket::ItemStackRequest() => {
                unimplemented!()
            }
            GamePacket::ItemStackResponse() => {
                unimplemented!()
            }
            GamePacket::UpdatePlayerGameType() => {
                unimplemented!()
            }
            GamePacket::PacketViolationWarning() => {
                unimplemented!()
            }
            GamePacket::ItemComponent() => {
                unimplemented!()
            }
            GamePacket::FilterTextPacket() => {
                unimplemented!()
            }
            GamePacket::UpdateSubChunkBlocksPacket() => {
                unimplemented!()
            }
            GamePacket::SubChunkPacket() => {
                unimplemented!()
            }
            GamePacket::SubChunkRequestPacket() => {
                unimplemented!()
            }
            GamePacket::DimensionData() => {
                unimplemented!()
            }
            GamePacket::RequestNetworkSettings(pk) => {
                ser_packet!(buf, GamePacket::RequestNetworkSettingsID, pk)
            }
            GamePacket::AlexEntityAnimation() => {
                unimplemented!()
            }
        }
    }

    pub fn pk_deserialize(
        cursor: &mut Cursor<Vec<u8>>,
    ) -> Result<(GamePacket, u8, u8), ProtoCodecError> {
        // Read the game packet length
        // We don't need it, yet
        // TODO: Use this to possibly async the packet handling
        match cursor.read_u32_varint() {
            Ok(_) => {}
            Err(e) => return Err(ProtoCodecError::IOError(e)),
        };

        // Read the game packet header
        let game_packet_header = match cursor.read_u16_varint() {
            Ok(v) => v,
            Err(e) => return Err(ProtoCodecError::IOError(e)),
        };

        // Get the first 10 bits as the packet id
        // Can never be more than a 16-bit integer due to being 10 bits big
        let game_packet_id = game_packet_header & 0b0000_0011_1111_1111;

        // Get the next 2 bits as the sub client sender id
        // Can never be more than an 8-bit integer due to being 2 bits big
        let sub_client_sender_id = (game_packet_header & 0b0000_1100_0000_0000) as u8;
        // Get the next 2 bits as the sub client target id
        // Can never be more than an 8-bit integer due to being 2 bits big
        let sub_client_target_id = (game_packet_header & 0b0011_0000_0000_0000) as u8;

        // Parse the game packet id into a valid GamePacket enum variant
        // let game_packet_id = match GamePacket::from_u16(game_packet_id) {
        //     Some(id) => id,
        //     None => return Err(ProtoCodecError::InvalidGamePacket(game_packet_id)),
        // };

        // Match the GamePacket to deserialize the correct packet type
        let game_packet = match game_packet_id {
            GamePacket::LoginID => GamePacket::Login(de_packet!(cursor, LoginPacket)),
            GamePacket::PlayStatusID => {
                GamePacket::PlayStatus(de_packet!(cursor, PlayStatusPacket))
            }
            GamePacket::ServerToClientHandshakeID => GamePacket::ServerToClientHandshake(
                de_packet!(cursor, HandshakeServerToClientPacket),
            ),
            GamePacket::ClientToServerHandshakeID => {
                unimplemented!()
            }
            GamePacket::DisconnectID => {
                GamePacket::Disconnect(de_packet!(cursor, DisconnectPacket))
            }
            GamePacket::ResourcePacksInfoID => {
                GamePacket::ResourcePacksInfo(de_packet!(cursor, ResourcePacksInfoPacket))
            }
            GamePacket::ResourcePacksStackID => {
                GamePacket::ResourcePackStack(de_packet!(cursor, ResourcePacksStackPacket))
            }
            GamePacket::ResourcePacksClientResponseID => GamePacket::ResourcePackClientResponse(
                de_packet!(cursor, ResourcePacksResponsePacket),
            ),
            GamePacket::TextID => {
                unimplemented!()
            }
            GamePacket::SetTimeID => {
                unimplemented!()
            }
            GamePacket::StartGameID => {
                unimplemented!()
            }
            GamePacket::AddPlayerID => {
                unimplemented!()
            }
            GamePacket::AddEntityID => {
                unimplemented!()
            }
            GamePacket::RemoveEntityID => {
                unimplemented!()
            }
            GamePacket::AddItemEntityID => {
                unimplemented!()
            }
            GamePacket::TakeItemEntityID => {
                unimplemented!()
            }
            GamePacket::MoveEntityID => {
                unimplemented!()
            }
            GamePacket::MovePlayerID => {
                unimplemented!()
            }
            GamePacket::RiderJumpID => {
                unimplemented!()
            }
            GamePacket::UpdateBlockID => {
                unimplemented!()
            }
            GamePacket::AddPaintingID => {
                unimplemented!()
            }
            GamePacket::TickSyncID => {
                unimplemented!()
            }
            GamePacket::LevelSoundEventOldID => {
                unimplemented!()
            }
            GamePacket::LevelEventID => {
                unimplemented!()
            }
            GamePacket::BlockEventID => {
                unimplemented!()
            }
            GamePacket::EntityEventID => {
                unimplemented!()
            }
            GamePacket::MobEffectID => {
                unimplemented!()
            }
            GamePacket::UpdateAttributesID => {
                unimplemented!()
            }
            GamePacket::InventoryTransactionID => {
                unimplemented!()
            }
            GamePacket::MobEquipmentID => {
                unimplemented!()
            }
            GamePacket::MobArmorEquipmentID => {
                unimplemented!()
            }
            GamePacket::InteractID => {
                unimplemented!()
            }
            GamePacket::BlockPickRequestID => {
                unimplemented!()
            }
            GamePacket::EntityPickRequestID => {
                unimplemented!()
            }
            GamePacket::PlayerActionID => {
                unimplemented!()
            }
            GamePacket::HurtArmorID => {
                unimplemented!()
            }
            GamePacket::SetEntityDataID => {
                unimplemented!()
            }
            GamePacket::SetEntityMotionID => {
                unimplemented!()
            }
            GamePacket::SetEntityLinkID => {
                unimplemented!()
            }
            GamePacket::SetHealthID => {
                unimplemented!()
            }
            GamePacket::SetSpawnPositionID => {
                unimplemented!()
            }
            GamePacket::AnimateID => {
                unimplemented!()
            }
            GamePacket::RespawnID => {
                unimplemented!()
            }
            GamePacket::ContainerOpenID => {
                unimplemented!()
            }
            GamePacket::ContainerCloseID => {
                unimplemented!()
            }
            GamePacket::PlayerHotbarID => {
                unimplemented!()
            }
            GamePacket::InventoryContentID => {
                unimplemented!()
            }
            GamePacket::InventorySlotID => {
                unimplemented!()
            }
            GamePacket::ContainerSetDataID => {
                unimplemented!()
            }
            GamePacket::CraftingDataID => {
                unimplemented!()
            }
            GamePacket::CraftingEventID => {
                unimplemented!()
            }
            GamePacket::GuiDataPickItemID => {
                unimplemented!()
            }
            GamePacket::AdventureSettingsID => {
                unimplemented!()
            }
            GamePacket::BlockEntityDataID => {
                unimplemented!()
            }
            GamePacket::PlayerInputID => {
                unimplemented!()
            }
            GamePacket::LevelChunkID => {
                unimplemented!()
            }
            GamePacket::SetCommandsEnabledID => {
                unimplemented!()
            }
            GamePacket::SetDifficultyID => {
                unimplemented!()
            }
            GamePacket::ChangeDimensionID => {
                unimplemented!()
            }
            GamePacket::SetPlayerGameTypeID => {
                unimplemented!()
            }
            GamePacket::PlayerListID => {
                unimplemented!()
            }
            GamePacket::SimpleEventID => {
                unimplemented!()
            }
            GamePacket::TelemetryEventID => {
                unimplemented!()
            }
            GamePacket::SpawnExperienceOrbID => {
                unimplemented!()
            }
            GamePacket::ClientboundMapItemDataID => {
                unimplemented!()
            }
            GamePacket::MapInfoRequestID => {
                unimplemented!()
            }
            GamePacket::RequestChunkRadiusID => {
                unimplemented!()
            }
            GamePacket::ChunkRadiusUpdateID => {
                unimplemented!()
            }
            GamePacket::ItemFrameDropItemID => {
                unimplemented!()
            }
            GamePacket::GameRulesChangedID => {
                unimplemented!()
            }
            GamePacket::CameraID => {
                unimplemented!()
            }
            GamePacket::BossEventID => {
                unimplemented!()
            }
            GamePacket::ShowCreditsID => {
                unimplemented!()
            }
            GamePacket::AvailableCommandsID => {
                unimplemented!()
            }
            GamePacket::CommandRequestID => {
                unimplemented!()
            }
            GamePacket::CommandBlockUpdateID => {
                unimplemented!()
            }
            GamePacket::CommandOutputID => {
                unimplemented!()
            }
            GamePacket::UpdateTradeID => {
                unimplemented!()
            }
            GamePacket::UpdateEquipmentID => {
                unimplemented!()
            }
            GamePacket::ResourcePackDataInfoID => {
                unimplemented!()
            }
            GamePacket::ResourcePackChunkDataID => {
                unimplemented!()
            }
            GamePacket::ResourcePackChunkRequestID => {
                unimplemented!()
            }
            GamePacket::TransferID => {
                unimplemented!()
            }
            GamePacket::PlaySoundID => {
                unimplemented!()
            }
            GamePacket::StopSoundID => {
                unimplemented!()
            }
            GamePacket::SetTitleID => {
                unimplemented!()
            }
            GamePacket::AddBehaviorTreeID => {
                unimplemented!()
            }
            GamePacket::StructureBlockUpdateID => {
                unimplemented!()
            }
            GamePacket::ShowStoreOfferID => {
                unimplemented!()
            }
            GamePacket::PurchaseReceiptID => {
                unimplemented!()
            }
            GamePacket::PlayerSkinID => {
                unimplemented!()
            }
            GamePacket::SubClientLoginID => {
                unimplemented!()
            }
            GamePacket::InitiateWebSocketConnectionID => {
                unimplemented!()
            }
            GamePacket::SetLastHurtByID => {
                unimplemented!()
            }
            GamePacket::BookEditID => {
                unimplemented!()
            }
            GamePacket::NpcRequestID => {
                unimplemented!()
            }
            GamePacket::PhotoTransferID => {
                unimplemented!()
            }
            GamePacket::ModalFormRequestID => {
                unimplemented!()
            }
            GamePacket::ModalFormResponseID => {
                unimplemented!()
            }
            GamePacket::ServerSettingsRequestID => {
                unimplemented!()
            }
            GamePacket::ServerSettingsResponseID => {
                unimplemented!()
            }
            GamePacket::ShowProfileID => {
                unimplemented!()
            }
            GamePacket::SetDefaultGameTypeID => {
                unimplemented!()
            }
            GamePacket::RemoveObjectiveID => {
                unimplemented!()
            }
            GamePacket::SetDisplayObjectiveID => {
                unimplemented!()
            }
            GamePacket::SetScoreID => {
                unimplemented!()
            }
            GamePacket::LabTableID => {
                unimplemented!()
            }
            GamePacket::UpdateBlockSyncedID => {
                unimplemented!()
            }
            GamePacket::MoveEntityDeltaID => {
                unimplemented!()
            }
            GamePacket::SetScoreboardIdentityID => {
                unimplemented!()
            }
            GamePacket::SetLocalPlayerAsInitializedID => {
                unimplemented!()
            }
            GamePacket::UpdateSoftEnumID => {
                unimplemented!()
            }
            GamePacket::NetworkStackLatencyID => {
                unimplemented!()
            }
            GamePacket::ScriptCustomEventID => {
                unimplemented!()
            }
            GamePacket::SpawnParticleEffectID => {
                unimplemented!()
            }
            GamePacket::AvailableEntityIdentifiersID => {
                unimplemented!()
            }
            GamePacket::LevelSoundEventV2ID => {
                unimplemented!()
            }
            GamePacket::NetworkChunkPublisherUpdateID => {
                unimplemented!()
            }
            GamePacket::BiomeDefinitionListID => {
                unimplemented!()
            }
            GamePacket::LevelSoundEventID => {
                unimplemented!()
            }
            GamePacket::LevelEventGenericID => {
                unimplemented!()
            }
            GamePacket::LecternUpdateID => {
                unimplemented!()
            }
            GamePacket::VideoStreamConnectID => {
                unimplemented!()
            }
            GamePacket::ClientCacheStatusID => {
                GamePacket::ClientCacheStatus(de_packet!(cursor, ClientCacheStatusPacket))
            }
            GamePacket::OnScreenTextureAnimationID => {
                unimplemented!()
            }
            GamePacket::MapCreateLockedCopyID => {
                unimplemented!()
            }
            GamePacket::StructureTemplateDataExportRequestID => {
                unimplemented!()
            }
            GamePacket::StructureTemplateDataExportResponseID => {
                unimplemented!()
            }
            GamePacket::UpdateBlockPropertiesID => {
                unimplemented!()
            }
            GamePacket::ClientCacheBlobStatusID => {
                unimplemented!()
            }
            GamePacket::ClientCacheMissResponseID => {
                unimplemented!()
            }
            GamePacket::NetworkSettingsID => {
                GamePacket::NetworkSettings(de_packet!(cursor, NetworkSettingsPacket))
            }
            GamePacket::PlayerAuthInputID => {
                unimplemented!()
            }
            GamePacket::CreativeContentID => {
                unimplemented!()
            }
            GamePacket::PlayerEnchantOptionsID => {
                unimplemented!()
            }
            GamePacket::ItemStackRequestID => {
                unimplemented!()
            }
            GamePacket::ItemStackResponseID => {
                unimplemented!()
            }
            GamePacket::UpdatePlayerGameTypeID => {
                unimplemented!()
            }
            GamePacket::PacketViolationWarningID => {
                unimplemented!()
            }
            GamePacket::ItemComponentID => {
                unimplemented!()
            }
            GamePacket::FilterTextPacketID => {
                unimplemented!()
            }
            GamePacket::UpdateSubChunkBlocksPacketID => {
                unimplemented!()
            }
            GamePacket::SubChunkPacketID => {
                unimplemented!()
            }
            GamePacket::SubChunkRequestPacketID => {
                unimplemented!()
            }
            GamePacket::DimensionDataID => {
                unimplemented!()
            }
            GamePacket::RequestNetworkSettingsID => {
                GamePacket::RequestNetworkSettings(de_packet!(cursor, NetworkSettingsRequestPacket))
            }
            GamePacket::AlexEntityAnimationID => {
                unimplemented!()
            }
            _ => {
                return Err(ProtoCodecError::InvalidEnumID);
            }
        };

        Ok((game_packet, sub_client_sender_id, sub_client_target_id))
    }
}
