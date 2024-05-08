use std::io::{Cursor, Write};

use num_traits::FromPrimitive;
use varint_rs::{VarintReader, VarintWriter};

use serialize::error::{DeserilizationError, SerilizationError};
use serialize::proto::de::MCProtoDeserialize;
use serialize::proto::ser::MCProtoSerialize;

use crate::info::GamePacketID;
use crate::packets::handshake_server_to_client::HandshakeServerToClientPacket;
use crate::packets::login::LoginPacket;
use crate::packets::network_settings::NetworkSettingsPacket;
use crate::packets::network_settings_request::NetworkSettingsRequestPacket;
use crate::packets::play_status::PlayStatusPacket;

#[repr(u64)]
#[derive(Debug)]
pub enum GamePacket {
    Login(LoginPacket),
    PlayStatus(PlayStatusPacket),
    ServerToClientHandshake(HandshakeServerToClientPacket),
    ClientToServerHandshake(),
    Disconnect(),
    ResourcePacksInfo(),
    ResourcePackStack(),
    ResourcePackClientResponse(),
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
    ClientCacheStatus(),
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

macro_rules! ser_packet {
    ($buf:expr, $packet_id:expr, $packet_data:expr) => {{
        let mut pk_buf = vec![];

        // Write the PacketID to the packet buffer
        match pk_buf.write_u64_varint($packet_id as u64) {
            Ok(_) => {}
            Err(e) => {
                return Err(SerilizationError::WriteIOError);
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
        match $buf.write_u64_varint(pk_buf.len() as u64) {
            Ok(_) => {}
            Err(_) => {
                return Err(SerilizationError::WriteIOError);
            }
        }

        // Copy pk buffer into buffer
        match $buf.write_all(&*pk_buf) {
            Ok(_) => {}
            Err(_) => {
                return Err(SerilizationError::WriteIOError);
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
    pub fn pk_serilize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError> {
        match self {
            GamePacket::Login(pk) => {
                ser_packet!(buf, GamePacketID::LoginID, pk)
            }
            GamePacket::PlayStatus(pk) => {
                ser_packet!(buf, GamePacketID::PlayStatusID, pk)
            }
            GamePacket::ServerToClientHandshake(pk) => {
                ser_packet!(buf, GamePacketID::ServerToClientHandshakeID, pk)
            }
            GamePacket::ClientToServerHandshake() => {
                unimplemented!()
            }
            GamePacket::Disconnect() => {
                unimplemented!()
            }
            GamePacket::ResourcePacksInfo() => {
                unimplemented!()
            }
            GamePacket::ResourcePackStack() => {
                unimplemented!()
            }
            GamePacket::ResourcePackClientResponse() => {
                unimplemented!()
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
            GamePacket::ClientCacheStatus() => {
                unimplemented!()
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
                ser_packet!(buf, GamePacketID::NetworkSettingsID, pk)
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
                ser_packet!(buf, GamePacketID::RequestNetworkSettingsID, pk)
            }
            GamePacket::AlexEntityAnimation() => {
                unimplemented!()
            }
        }
    }

    pub fn pk_deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<GamePacket, DeserilizationError> {
        // Read the gamepacket length
        // We don't need it
        match cursor.read_u64_varint() {
            Ok(_) => {}
            Err(_) => return Err(DeserilizationError::ReadIOError),
        };

        // Read the gamepacket ID
        let gamepacket_id: GamePacketID = match cursor.read_u64_varint() {
            Ok(v) => { match GamePacketID::from_u64(v) {
                    Some(pk) => pk,
                    None => return Err(DeserilizationError::InvalidGamepacketID),
            }}
            Err(_) => return Err(DeserilizationError::ReadIOError),
        };

        match gamepacket_id {
            GamePacketID::LoginID => Ok(GamePacket::Login(de_packet!(cursor, LoginPacket))),
            GamePacketID::PlayStatusID => {
                Ok(GamePacket::PlayStatus(
                    de_packet!(cursor, PlayStatusPacket),
                ))
            }
            GamePacketID::ServerToClientHandshakeID => Ok(GamePacket::ServerToClientHandshake(
                de_packet!(cursor, HandshakeServerToClientPacket),
            )),
            GamePacketID::ClientToServerHandshakeID => {
                unimplemented!()
            }
            GamePacketID::DisconnectID => {
                unimplemented!()
            }
            GamePacketID::ResourcePacksInfoID => {
                unimplemented!()
            }
            GamePacketID::ResourcePackStackID => {
                unimplemented!()
            }
            GamePacketID::ResourcePackClientResponseID => {
                unimplemented!()
            }
            GamePacketID::TextID => {
                unimplemented!()
            }
            GamePacketID::SetTimeID => {
                unimplemented!()
            }
            GamePacketID::StartGameID => {
                unimplemented!()
            }
            GamePacketID::AddPlayerID => {
                unimplemented!()
            }
            GamePacketID::AddEntityID => {
                unimplemented!()
            }
            GamePacketID::RemoveEntityID => {
                unimplemented!()
            }
            GamePacketID::AddItemEntityID => {
                unimplemented!()
            }
            GamePacketID::TakeItemEntityID => {
                unimplemented!()
            }
            GamePacketID::MoveEntityID => {
                unimplemented!()
            }
            GamePacketID::MovePlayerID => {
                unimplemented!()
            }
            GamePacketID::RiderJumpID => {
                unimplemented!()
            }
            GamePacketID::UpdateBlockID => {
                unimplemented!()
            }
            GamePacketID::AddPaintingID => {
                unimplemented!()
            }
            GamePacketID::TickSyncID => {
                unimplemented!()
            }
            GamePacketID::LevelSoundEventOldID => {
                unimplemented!()
            }
            GamePacketID::LevelEventID => {
                unimplemented!()
            }
            GamePacketID::BlockEventID => {
                unimplemented!()
            }
            GamePacketID::EntityEventID => {
                unimplemented!()
            }
            GamePacketID::MobEffectID => {
                unimplemented!()
            }
            GamePacketID::UpdateAttributesID => {
                unimplemented!()
            }
            GamePacketID::InventoryTransactionID => {
                unimplemented!()
            }
            GamePacketID::MobEquipmentID => {
                unimplemented!()
            }
            GamePacketID::MobArmorEquipmentID => {
                unimplemented!()
            }
            GamePacketID::InteractID => {
                unimplemented!()
            }
            GamePacketID::BlockPickRequestID => {
                unimplemented!()
            }
            GamePacketID::EntityPickRequestID => {
                unimplemented!()
            }
            GamePacketID::PlayerActionID => {
                unimplemented!()
            }
            GamePacketID::HurtArmorID => {
                unimplemented!()
            }
            GamePacketID::SetEntityDataID => {
                unimplemented!()
            }
            GamePacketID::SetEntityMotionID => {
                unimplemented!()
            }
            GamePacketID::SetEntityLinkID => {
                unimplemented!()
            }
            GamePacketID::SetHealthID => {
                unimplemented!()
            }
            GamePacketID::SetSpawnPositionID => {
                unimplemented!()
            }
            GamePacketID::AnimateID => {
                unimplemented!()
            }
            GamePacketID::RespawnID => {
                unimplemented!()
            }
            GamePacketID::ContainerOpenID => {
                unimplemented!()
            }
            GamePacketID::ContainerCloseID => {
                unimplemented!()
            }
            GamePacketID::PlayerHotbarID => {
                unimplemented!()
            }
            GamePacketID::InventoryContentID => {
                unimplemented!()
            }
            GamePacketID::InventorySlotID => {
                unimplemented!()
            }
            GamePacketID::ContainerSetDataID => {
                unimplemented!()
            }
            GamePacketID::CraftingDataID => {
                unimplemented!()
            }
            GamePacketID::CraftingEventID => {
                unimplemented!()
            }
            GamePacketID::GuiDataPickItemID => {
                unimplemented!()
            }
            GamePacketID::AdventureSettingsID => {
                unimplemented!()
            }
            GamePacketID::BlockEntityDataID => {
                unimplemented!()
            }
            GamePacketID::PlayerInputID => {
                unimplemented!()
            }
            GamePacketID::LevelChunkID => {
                unimplemented!()
            }
            GamePacketID::SetCommandsEnabledID => {
                unimplemented!()
            }
            GamePacketID::SetDifficultyID => {
                unimplemented!()
            }
            GamePacketID::ChangeDimensionID => {
                unimplemented!()
            }
            GamePacketID::SetPlayerGameTypeID => {
                unimplemented!()
            }
            GamePacketID::PlayerListID => {
                unimplemented!()
            }
            GamePacketID::SimpleEventID => {
                unimplemented!()
            }
            GamePacketID::TelemetryEventID => {
                unimplemented!()
            }
            GamePacketID::SpawnExperienceOrbID => {
                unimplemented!()
            }
            GamePacketID::ClientboundMapItemDataID => {
                unimplemented!()
            }
            GamePacketID::MapInfoRequestID => {
                unimplemented!()
            }
            GamePacketID::RequestChunkRadiusID => {
                unimplemented!()
            }
            GamePacketID::ChunkRadiusUpdateID => {
                unimplemented!()
            }
            GamePacketID::ItemFrameDropItemID => {
                unimplemented!()
            }
            GamePacketID::GameRulesChangedID => {
                unimplemented!()
            }
            GamePacketID::CameraID => {
                unimplemented!()
            }
            GamePacketID::BossEventID => {
                unimplemented!()
            }
            GamePacketID::ShowCreditsID => {
                unimplemented!()
            }
            GamePacketID::AvailableCommandsID => {
                unimplemented!()
            }
            GamePacketID::CommandRequestID => {
                unimplemented!()
            }
            GamePacketID::CommandBlockUpdateID => {
                unimplemented!()
            }
            GamePacketID::CommandOutputID => {
                unimplemented!()
            }
            GamePacketID::UpdateTradeID => {
                unimplemented!()
            }
            GamePacketID::UpdateEquipmentID => {
                unimplemented!()
            }
            GamePacketID::ResourcePackDataInfoID => {
                unimplemented!()
            }
            GamePacketID::ResourcePackChunkDataID => {
                unimplemented!()
            }
            GamePacketID::ResourcePackChunkRequestID => {
                unimplemented!()
            }
            GamePacketID::TransferID => {
                unimplemented!()
            }
            GamePacketID::PlaySoundID => {
                unimplemented!()
            }
            GamePacketID::StopSoundID => {
                unimplemented!()
            }
            GamePacketID::SetTitleID => {
                unimplemented!()
            }
            GamePacketID::AddBehaviorTreeID => {
                unimplemented!()
            }
            GamePacketID::StructureBlockUpdateID => {
                unimplemented!()
            }
            GamePacketID::ShowStoreOfferID => {
                unimplemented!()
            }
            GamePacketID::PurchaseReceiptID => {
                unimplemented!()
            }
            GamePacketID::PlayerSkinID => {
                unimplemented!()
            }
            GamePacketID::SubClientLoginID => {
                unimplemented!()
            }
            GamePacketID::InitiateWebSocketConnectionID => {
                unimplemented!()
            }
            GamePacketID::SetLastHurtByID => {
                unimplemented!()
            }
            GamePacketID::BookEditID => {
                unimplemented!()
            }
            GamePacketID::NpcRequestID => {
                unimplemented!()
            }
            GamePacketID::PhotoTransferID => {
                unimplemented!()
            }
            GamePacketID::ModalFormRequestID => {
                unimplemented!()
            }
            GamePacketID::ModalFormResponseID => {
                unimplemented!()
            }
            GamePacketID::ServerSettingsRequestID => {
                unimplemented!()
            }
            GamePacketID::ServerSettingsResponseID => {
                unimplemented!()
            }
            GamePacketID::ShowProfileID => {
                unimplemented!()
            }
            GamePacketID::SetDefaultGameTypeID => {
                unimplemented!()
            }
            GamePacketID::RemoveObjectiveID => {
                unimplemented!()
            }
            GamePacketID::SetDisplayObjectiveID => {
                unimplemented!()
            }
            GamePacketID::SetScoreID => {
                unimplemented!()
            }
            GamePacketID::LabTableID => {
                unimplemented!()
            }
            GamePacketID::UpdateBlockSyncedID => {
                unimplemented!()
            }
            GamePacketID::MoveEntityDeltaID => {
                unimplemented!()
            }
            GamePacketID::SetScoreboardIdentityID => {
                unimplemented!()
            }
            GamePacketID::SetLocalPlayerAsInitializedID => {
                unimplemented!()
            }
            GamePacketID::UpdateSoftEnumID => {
                unimplemented!()
            }
            GamePacketID::NetworkStackLatencyID => {
                unimplemented!()
            }
            GamePacketID::ScriptCustomEventID => {
                unimplemented!()
            }
            GamePacketID::SpawnParticleEffectID => {
                unimplemented!()
            }
            GamePacketID::AvailableEntityIdentifiersID => {
                unimplemented!()
            }
            GamePacketID::LevelSoundEventV2ID => {
                unimplemented!()
            }
            GamePacketID::NetworkChunkPublisherUpdateID => {
                unimplemented!()
            }
            GamePacketID::BiomeDefinitionListID => {
                unimplemented!()
            }
            GamePacketID::LevelSoundEventID => {
                unimplemented!()
            }
            GamePacketID::LevelEventGenericID => {
                unimplemented!()
            }
            GamePacketID::LecternUpdateID => {
                unimplemented!()
            }
            GamePacketID::VideoStreamConnectID => {
                unimplemented!()
            }
            GamePacketID::ClientCacheStatusID => {
                unimplemented!()
            }
            GamePacketID::OnScreenTextureAnimationID => {
                unimplemented!()
            }
            GamePacketID::MapCreateLockedCopyID => {
                unimplemented!()
            }
            GamePacketID::StructureTemplateDataExportRequestID => {
                unimplemented!()
            }
            GamePacketID::StructureTemplateDataExportResponseID => {
                unimplemented!()
            }
            GamePacketID::UpdateBlockPropertiesID => {
                unimplemented!()
            }
            GamePacketID::ClientCacheBlobStatusID => {
                unimplemented!()
            }
            GamePacketID::ClientCacheMissResponseID => {
                unimplemented!()
            }
            GamePacketID::NetworkSettingsID => Ok(GamePacket::NetworkSettings(de_packet!(
                cursor,
                NetworkSettingsPacket
            ))),
            GamePacketID::PlayerAuthInputID => {
                unimplemented!()
            }
            GamePacketID::CreativeContentID => {
                unimplemented!()
            }
            GamePacketID::PlayerEnchantOptionsID => {
                unimplemented!()
            }
            GamePacketID::ItemStackRequestID => {
                unimplemented!()
            }
            GamePacketID::ItemStackResponseID => {
                unimplemented!()
            }
            GamePacketID::UpdatePlayerGameTypeID => {
                unimplemented!()
            }
            GamePacketID::PacketViolationWarningID => {
                unimplemented!()
            }
            GamePacketID::ItemComponentID => {
                unimplemented!()
            }
            GamePacketID::FilterTextPacketID => {
                unimplemented!()
            }
            GamePacketID::UpdateSubChunkBlocksPacketID => {
                unimplemented!()
            }
            GamePacketID::SubChunkPacketID => {
                unimplemented!()
            }
            GamePacketID::SubChunkRequestPacketID => {
                unimplemented!()
            }
            GamePacketID::DimensionDataID => {
                unimplemented!()
            }
            GamePacketID::RequestNetworkSettingsID => Ok(GamePacket::RequestNetworkSettings(
                de_packet!(cursor, NetworkSettingsRequestPacket),
            )),
            GamePacketID::AlexEntityAnimationID => {
                unimplemented!()
            }
        }
    }
}
