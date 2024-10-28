use crate::version::v729::packets::handshake_client_to_server::HandshakeClientToServerPacket;
use crate::version::v729::packets::{
    add_actor::AddActorPacket, add_painting::AddPaintingPacket, add_player::AddPlayerPacket,
    animate_player::AnimatePlayerPacket,
    available_entity_identifiers::AvailableEntityIdentifiersPacket,
    award_achievement::AwardAchievementPacket, boss_event::BossEventPacket, camera::CameraPacket,
    camera_shake::CameraShakePacket, change_dimension::ChangeDimensionPacket,
    chunk_publisher_update::ChunkPublisherUpdatePacket,
    chunk_radius_request::ChunkRadiusRequestPacket, chunk_radius_updated::ChunkRadiusUpdatedPacket,
    client_cache_status::ClientCacheStatusPacket, command_request::CommandRequestPacket,
    container_close::ContainerClosePacket, container_open::ContainerOpenPacket,
    correct_player_move_prediction::CorrectPlayerMovePredictionPacket, debug_info::DebugInfoPacket,
    emote::EmotePacket, emote_list::EmoteListPacket,
    handshake_server_to_client::HandshakeServerToClientPacket, interact::InteractPacket,
    inventory_content::InventoryContentPacket, level_chunk::LevelChunkPacket,
    loading_screen::LoadingScreenPacket, login::LoginPacket, mob_equipment::MobEquipmentPacket,
    modal_form_request::ModalFormRequestPacket, modal_form_response::ModalFormResponsePacket,
    network_settings::NetworkSettingsPacket,
    network_settings_request::NetworkSettingsRequestPacket, open_sign::OpenSignPacket,
    packet_violation_warning::PacketViolationWarningPacket, play_status::PlayStatusPacket,
    player_action::PlayerActionPacket, player_auth_input::PlayerAuthInputPacket,
    player_disconnect::DisconnectPlayerPacket, player_hotbar::PlayerHotbarPacket,
    player_move::MovePlayerPacket, player_transfer::TransferPlayerPacket,
    remove_actor::RemoveEntityPacket, resource_packs_info::ResourcePacksInfoPacket,
    resource_packs_response::ResourcePacksResponsePacket,
    resource_packs_stack::ResourcePacksStackPacket, respawn::RespawnPacket,
    server_player_post_move_position::ServerPlayerPostMovePositionPacket,
    server_settings_request::ServerSettingsRequestPacket,
    server_settings_response::ServerSettingsResponsePacket,
    set_commands_enabled::SetCommandsEnabledPacket,
    set_local_player_as_initialized::SetLocalPlayerAsInitializedPacket, set_time::SetTimePacket,
    set_title::SetTitlePacket, show_credits::ShowCreditsPacket, show_profile::ShowProfilePacket,
    start_game::StartGamePacket, text_message::TextMessagePacket,
    toast_request::ToastRequestPacket, update_difficulty::UpdateDifficultyPacket,
    update_player_gamemode::UpdatePlayerGamemodePacket,
};
use bedrockrs_macros::gamepackets;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::sub_client::SubClientID;
use std::io::{Cursor, Write};
use varint_rs::{VarintReader, VarintWriter};

gamepackets! {
    Login: LoginPacket,
    PlayStatus: PlayStatusPacket,
    HandshakeServerToClient: HandshakeServerToClientPacket,
    HandshakeClientToServer: HandshakeClientToServerPacket,
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
    AvailableEntityIdentifiers: AvailableEntityIdentifiersPacket,
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
    Emote: EmotePacket,
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
    BiomeDefinitionListCompressed: _,
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
    let gamepacket_header: u16 = stream.read_u16_varint()?;

    // Get the first 10 bits as the packet id
    // Can never be more than a 16-bit integer due to being 10-bits big
    // Gamepacket IDs through 200-299 are used for spin-offs, they are free to use for custom packets
    let gamepacket_id = gamepacket_header & 0b0000_0011_1111_1111;

    // Get the next 2 bits as the sub client sender id
    // Can never be more than an 8-bit integer due to being 2 bits big
    let subclient_sender_id =
        SubClientID::try_from(((gamepacket_header & 0b0000_1100_0000_0000) >> 10) as u8)?;
    // Get the next 2 bits as the sub client target id
    // Never more than an 8-bit integer due to being 2 bits big
    let subclient_target_id =
        SubClientID::try_from(((gamepacket_header & 0b0011_0000_0000_0000) >> 12) as u8)?;

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
    let mut gamepacket_header: u16 = 0;

    // Set the first 10 bits as the packet id
    // Can never be more than a 16-bit integer due to being 10-bits big
    // Gamepacket IDs through 200-299 are used for spin-offs, they are free to use for custom packets
    gamepacket_header |= 0b0000_0011_1111_1111 & gamepacket_id;

    // Set the next 2 bits as the sub client sender id
    // Never more than an 8-bit integer due to being 2 bits big
    gamepacket_header |= (Into::<u16>::into(subclient_sender_id) >> 10) & 0b0000_1100_0000_0000;
    // Set the next 2 bits as the sub client target id
    // Never more than an 8-bit integer due to being 2 bits big
    gamepacket_header |= (Into::<u16>::into(subclient_target_id) >> 12) & 0b0011_0000_0000_0000;

    // Since the size of the header is also included in the batched packet size,
    // we need to write it to a temporary buffer
    let mut gamepacket_header_buf = Vec::new();

    // Write the gamepacket header into temporary buffer
    gamepacket_header_buf.write_u16_varint(gamepacket_header)?;

    // Write the gamepacket length and the header length
    stream.write_u32_varint(length + gamepacket_header_buf.len() as u32)?;

    // Write the final game packet header
    stream.write_all(gamepacket_header_buf.as_slice())?;

    Ok(())
}

const fn get_gamepacket_header_size_prediction() -> usize {
    // 2 = gamepacket header (varint u32, only 14 bites can be treated as an u16)
    // 4 = gamepacket length size (varint u32)
    2 + 4
}
