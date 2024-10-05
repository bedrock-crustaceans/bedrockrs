#![allow(non_upper_case_globals)]

use crate::packets::add_actor::AddActorPacket;
use crate::packets::add_painting::AddPaintingPacket;
use crate::packets::add_player::AddPlayerPacket;
use crate::packets::animate_player::AnimatePlayerPacket;
use crate::packets::boss_event::BossEventPacket;
use crate::packets::camera::CameraPacket;
use crate::packets::change_dimension::ChangeDimensionPacket;
use crate::packets::chunk_publisher_update::ChunkPublisherUpdatePacket;
use crate::packets::chunk_radius_request::ChunkRadiusRequestPacket;
use crate::packets::chunk_radius_updated::ChunkRadiusUpdatedPacket;
use crate::packets::client_cache_status::ClientCacheStatusPacket;
use crate::packets::command_request::CommandRequestPacket;
use crate::packets::container_close::ContainerClosePacket;
use crate::packets::container_open::ContainerOpenPacket;
use crate::packets::correct_player_move_prediction::CorrectPlayerMovePredictionPacket;
use crate::packets::debug_info::DebugInfoPacket;
use crate::packets::emote_list::EmoteListPacket;
use crate::packets::handshake_server_to_client::HandshakeServerToClientPacket;
use crate::packets::interact::InteractPacket;
use crate::packets::inventory_content::InventoryContentPacket;
use crate::packets::level_chunk::LevelChunkPacket;
use crate::packets::loading_screen::LoadingScreenPacket;
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
use crate::packets::player_disconnect::DisconnectPlayerPacket;
use crate::packets::player_hotbar::PlayerHotbarPacket;
use crate::packets::player_move::MovePlayerPacket;
use crate::packets::player_transfer::TransferPlayerPacket;
use crate::packets::remove_actor::RemoveEntityPacket;
use crate::packets::resource_packs_info::ResourcePacksInfoPacket;
use crate::packets::resource_packs_response::ResourcePacksResponsePacket;
use crate::packets::resource_packs_stack::ResourcePacksStackPacket;
use crate::packets::respawn::RespawnPacket;
use crate::packets::server_player_post_move_position::ServerPlayerPostMovePositionPacket;
use crate::packets::server_settings_request::ServerSettingsRequestPacket;
use crate::packets::server_settings_response::ServerSettingsResponsePacket;
use crate::packets::set_commands_enabled::SetCommandsEnabledPacket;
use crate::packets::update_difficulty::UpdateDifficultyPacket;
use crate::packets::set_local_player_as_initialized::SetLocalPlayerAsInitializedPacket;
use crate::packets::update_player_gamemode::UpdatePlayerGamemodePacket;
use crate::packets::set_time::SetTimePacket;
use crate::packets::set_title::SetTitlePacket;
use crate::packets::show_credits::ShowCreditsPacket;
use crate::packets::show_profile::ShowProfilePacket;
use crate::packets::start_game::StartGamePacket;
use crate::packets::text_message::TextMessagePacket;
use crate::packets::toast_request::ToastRequestPacket;
use crate::sub_client::SubClientID;
use bedrockrs_macros::gamepackets;
use bedrockrs_proto_core::{error::ProtoCodecError, GamePacket, ProtoCodec};
use std::io::{Cursor, Write};
use varint_rs::{VarintReader, VarintWriter};
use crate::packets::award_achievement::AwardAchievementPacket;
use crate::packets::camera_shake::CameraShakePacket;
use crate::packets::open_sign::OpenSignPacket;

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
    BlockActorData: _,
    PlayerInput: _,
    LevelChunk: LevelChunkPacket,
    SetCommandsEnabled: SetCommandsEnabledPacket,
    UpdateDifficulty: UpdateDifficultyPacket,
    ChangeDimension: ChangeDimensionPacket,
    UpdatePlayerGamemode: UpdatePlayerGamemodePacket,
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
    UpdateEquip: _,
    ResourcePackDataInfo: _,
    ResourcePackChunkData: _,
    ResourcePackChunkRequest: _,
    TransferPlayer: TransferPlayerPacket,
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
    SetDefaultGamemode: _,
    RemoveObjective: _,
    SetDisplayObjective: _,
    SetScore: _,
    LabTable: _,
    UpdateBlockSynced: _,
    MoveEntityDelta: _,
    SetScoreboardIdentity: _,
    SetLocalPlayerAsInitialized: SetLocalPlayerAsInitializedPacket,
    UpdateSoftEnum: _,
    Ping: _,
    SpawnParticleEffect: _,
    AvailableEntityIdentifiers: _,
    LevelSoundEventV2: _,
    ChunkPublisherUpdate: ChunkPublisherUpdatePacket,
    BiomeDefinitionList: _,
    LevelSoundEvent: _,
    LevelEventGeneric: _,
    LecternUpdate: _,
    ClientCacheStatus: ClientCacheStatusPacket,
    OnScreenTextureAnimation: _,
    CreateLockedMap: _,
    StructureDataRequest: _,
    StructureDataResponse: _,
    ClientCacheBlobStatus: _,
    ClientCacheMissResponse: _,
    EducationSettings: _,
    Emote: _,
    MultiplayerSettings: _,
    SettingsCommand: _,
    AnvilDamage: _,
    CompletedUsingItem: _,
    NetworkSettings: NetworkSettingsPacket,
    PlayerAuthInput: PlayerAuthInputPacket,
    CreativeContent: _,
    PlayerEnchantOptions: _,
    ItemStackRequest: _,
    ItemStackResponse: _,
    PlayerArmorDamage: _,
    CodeBuilder: _,
    // TODO: Find a better name for this, else this collides with `UpdatePlayerGamemode`
    UpdateOtherPlayerGamemode: _,
    EmoteList: EmoteListPacket,
    PositionTrackingDBServerBroadcast: _,
    PositionTrackingDBClientRequest: _,
    DebugInfo: DebugInfoPacket,
    PacketViolationWarning: PacketViolationWarningPacket,
    MotionPredictionHints: _,
    AnimateEntity: _,
    CameraShake: CameraShakePacket,
    PlayerFog: _,
    CorrectPlayerMovePrediction: CorrectPlayerMovePredictionPacket,
    ItemComponent: _,
    DebugRenderer: _,
    SyncEntityProperty: _,
    AddVolumeEntity: _,
    RemoveVolumeEntity: _,
    SimulationType: _,
    NpcDialogue:_,
    EduUriResource: _,
    CreatePhoto: _,
    UpdateSubChunkBlocksPacket: _,
    SubChunk: _,
    SubChunkRequest: _,
    PlayerStartItemCooldown: _,
    ScriptMessage: _,
    CodeBuilderSource: _,
    TickingAreasLoadStatus: _,
    DimensionData: _,
    AgentActionEvent: _,
    ChangeMobProperty: _,
    LessonProgress: _,
    RequestAbility: _,
    RequestPermissions: _,
    ToastRequestPacket: ToastRequestPacket,
    UpdateAbilities: _,
    UpdateAdventureSettings: _,
    DeathInfo: _,
    EditorNetwork: _,
    FeatureRegistry: _,
    ServerStats: _,
    NetworkSettingsRequest: NetworkSettingsRequestPacket,
    GameTestRequest: _,
    GameTestResults: _,
    UpdateClientInputLocks: _,
    CameraPresets: _,
    UnlockedRecipes: _,
    CameraInstruction: _,
    CompressedBiomeDefinitionList: _,
    TrimData: _,
    OpenSign: OpenSignPacket,
    AgentAnimation: _,
    RefreshEntitlements: _,
    PlayerToggleCrafterSlotRequest: _,
    SetPlayerInventoryOptions: _,
    SetHud: _,
    AwardAchievement: AwardAchievementPacket,
    CloseForm: _,
    LoadingScreen: LoadingScreenPacket,
    JigsawStructureData: _,
    CurrentStructureFeature: _,
    DiagnosticsPacket: _,
    CameraAimAssist: _,
    DynamicContainerRegistryCleanup: _,
}

fn read_gamepacket_header(
    stream: &mut Cursor<&[u8]>,
) -> Result<(u32, u16, SubClientID, SubClientID), ProtoCodecError> {
    // Read the gamepacket length
    let length = stream.read_u32_varint()?;

    // Read the gamepacket header and parse it into an u16
    // Since the (var)int is only storing 14 bytes we can treat it as an u16
    // This is normally treated as u32 varint
    let game_packet_header: u16 = stream.read_u16_varint()?;

    // Get the first 10 bits as the packet id
    // Can never be more than a 16-bit integer due to being 10-bits big
    // Gamepacket IDs through 200-299 are used for spin-offs, they are free to use for custom packets
    let gamepacket_id = game_packet_header & 0b0000_0011_1111_1111;

    // Get the next 2 bits as the sub client sender id
    // Can never be more than an 8-bit integer due to being 2 bits big
    let subclient_sender_id =
        SubClientID::proto_from(((game_packet_header & 0b0000_1100_0000_0000) >> 10) as u8)?;
    // Get the next 2 bits as the sub client target id
    // Never more than an 8-bit integer due to being 2 bits big
    let subclient_target_id =
        SubClientID::proto_from(((game_packet_header & 0b0011_0000_0000_0000) >> 12) as u8)?;

    Ok((
        length,
        gamepacket_id,
        subclient_sender_id,
        subclient_target_id,
    ))
}

fn write_gamepacket_header(
    stream: &mut Vec<u8>,
    length: u32,
    gamepacket_id: u16,
    subclient_sender_id: SubClientID,
    subclient_target_id: SubClientID,
) -> Result<(), ProtoCodecError> {
    // Since the (var)int is only storing 14 bytes, we can treat it as an u16
    // This is normally treated as u32 varint
    let mut game_packet_header: u16 = 0;

    // Set the first 10 bits as the packet id
    // Can never be more than a 16-bit integer due to being 10-bits big
    // Gamepacket IDs through 200-299 are used for spin-offs, they are free to use for custom packets
    game_packet_header |= 0b0000_0011_1111_1111 & gamepacket_id;

    // Set the next 2 bits as the sub client sender id
    // Never more than an 8-bit integer due to being 2 bits big
    game_packet_header |= (subclient_sender_id.proto_to() as u16 >> 10) & 0b0000_1100_0000_0000;
    // Set the next 2 bits as the sub client target id
    // Never more than an 8-bit integer due to being 2 bits big
    game_packet_header |= (subclient_target_id.proto_to() as u16 >> 12) & 0b0011_0000_0000_0000;

    // Since the size of the header is also included in the batched packet size,
    // we need to write it to a temporary buffer
    let mut game_packet_header_buf = Vec::new();

    // Write the gamepacket header into temporary buffer
    game_packet_header_buf.write_u16_varint(game_packet_header)?;

    // Write the gamepacket length and the header length
    stream.write_u32_varint(length + game_packet_header_buf.len() as u32)?;

    // Write the final game packet header
    stream.write_all(game_packet_header_buf.as_slice())?;

    Ok(())
}
