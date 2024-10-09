use bedrockrs_core::{Vec2, Vec3};
use bedrockrs_macros::{gamepacket, ProtoCodec};
use uuid::Uuid;

use crate::types::block_entry::BlockEntry;
use crate::types::item_entry::ItemEntry;
use crate::types::level_settings::LevelSettings;
use crate::types::network_permissions::NetworkPermissions;
use crate::types::player_movement_settings::PlayerMovementSettings;
use bedrockrs_shared::actor_runtime_id::ActorRuntimeID;
use bedrockrs_shared::actor_unique_id::ActorUniqueID;
use bedrockrs_shared::world::gamemode::Gamemode;

#[gamepacket(id = 11)]
#[derive(Debug, Clone)]
pub struct StartGamePacket {
    pub target_actor_id: ActorUniqueID,
    pub target_runtime_id: ActorRuntimeID,
    pub gamemode: Gamemode,
    //#[endianness(le)]
    pub position: Vec3<f32>,
    //#[endianness(le)]
    pub rotation: Vec2<f32>,
    pub settings: LevelSettings,
    pub level_id: String,
    pub level_name: String,
    pub template_content_identity: String,
    pub trial: bool,
    pub movement_settings: PlayerMovementSettings,
    //#[endianness(le)]
    pub current_level_time: u64,
    //#[endianness(var)]
    pub enchantment_seed: i32,
    /// List of all custom blocks registered on the server.
    //#[vec_repr(u32)]
    //#[vec_endianness(var)]
    pub blocks: Vec<BlockEntry>,
    /// List of all items with their legacy IDs that are available in the game.
    /// Failing to send any of the items that are in the game will crash mobile clients.
    //#[vec_repr(u32)]
    //#[vec_endianness(var)]
    pub items: Vec<ItemEntry>,
    pub multiplayer_correlation_id: String,
    pub enable_item_stack_net_manager: bool,
    pub server_version: String,
    // TODO: This can now be a concrete type rather than an NBT value.
    // How should we do this with the ProtoCodec macro?
    //#[nbt]
    pub player_property_data: nbtx::Value,
    //#[endianness(le)]
    pub block_type_registry_checksum: u64,
    pub world_template_id: Uuid,
    pub enable_clientside_world_generation: bool,
    pub use_block_network_id_hashes: bool,
    pub network_permission: NetworkPermissions,
}

impl ::bedrockrs_proto_core::ProtoCodec for StartGamePacket {
    fn proto_serialize(
        &self,
        stream: &mut Vec<u8>,
    ) -> Result<(), ::bedrockrs_proto_core::error::ProtoCodecError>
    where
        Self: Sized,
    {
        {
            let lvl = ::log::Level::Trace;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api::log(
                    format_args!("ProtoSerialize: {0}", "StartGamePacket"),
                    lvl,
                    &(
                        "bedrockrs_proto::packets::start_game",
                        "bedrockrs_proto::packets::start_game",
                        ::log::__private_api::loc(),
                    ),
                    (),
                );
            }
        };
        <ActorUniqueID as ::bedrockrs_proto_core::ProtoCodec>::proto_serialize(
            &self.target_actor_id,
            stream,
        )?;
        <ActorRuntimeID as ::bedrockrs_proto_core::ProtoCodec>::proto_serialize(
            &self.target_runtime_id,
            stream,
        )?;
        <Gamemode as ::bedrockrs_proto_core::ProtoCodec>::proto_serialize(
            &self.gamemode,
            stream,
        )?;
        <Vec3<
            f32,
        > as ::bedrockrs_proto_core::ProtoCodecLE>::proto_serialize(
            &self.position,
            stream,
        )?;
        <Vec2<
            f32,
        > as ::bedrockrs_proto_core::ProtoCodecLE>::proto_serialize(
            &self.rotation,
            stream,
        )?;
        <LevelSettings as ::bedrockrs_proto_core::ProtoCodec>::proto_serialize(
            &self.settings,
            stream,
        )?;
        <String as ::bedrockrs_proto_core::ProtoCodec>::proto_serialize(
            &self.level_id,
            stream,
        )?;
        <String as ::bedrockrs_proto_core::ProtoCodec>::proto_serialize(
            &self.level_name,
            stream,
        )?;
        <String as ::bedrockrs_proto_core::ProtoCodec>::proto_serialize(
            &self.template_content_identity,
            stream,
        )?;
        <bool as ::bedrockrs_proto_core::ProtoCodec>::proto_serialize(
            &self.trial,
            stream,
        )?;
        <PlayerMovementSettings as ::bedrockrs_proto_core::ProtoCodec>::proto_serialize(
            &self.movement_settings,
            stream,
        )?;
        <u64 as ::bedrockrs_proto_core::ProtoCodecLE>::proto_serialize(
            &self.current_level_time,
            stream,
        )?;
        <i32 as ::bedrockrs_proto_core::ProtoCodecVAR>::proto_serialize(
            &self.enchantment_seed,
            stream,
        )?;
        {
            let len: u32 = self.blocks.len().try_into()?;
            <u32 as ::bedrockrs_proto_core::ProtoCodecVAR>::proto_serialize(
                &len,
                stream,
            )?;
            for i in &self.blocks {
                <BlockEntry as ::bedrockrs_proto_core::ProtoCodec>::proto_serialize(
                    &i,
                    stream,
                )?;
            }
        };
        {
            let len: u32 = self.items.len().try_into()?;
            <u32 as ::bedrockrs_proto_core::ProtoCodecVAR>::proto_serialize(
                &len,
                stream,
            )?;
            for i in &self.items {
                <ItemEntry as ::bedrockrs_proto_core::ProtoCodec>::proto_serialize(
                    &i,
                    stream,
                )?;
            }
        };
        <String as ::bedrockrs_proto_core::ProtoCodec>::proto_serialize(
            &self.multiplayer_correlation_id,
            stream,
        )?;
        <bool as ::bedrockrs_proto_core::ProtoCodec>::proto_serialize(
            &self.enable_item_stack_net_manager,
            stream,
        )?;
        <String as ::bedrockrs_proto_core::ProtoCodec>::proto_serialize(
            &self.server_version,
            stream,
        )?;
        ::nbtx::to_bytes_in::<::nbtx::NetworkLittleEndian, >(
            &mut stream,
            &self.player_property_data
        )?;
        <u64 as ::bedrockrs_proto_core::ProtoCodecLE>::proto_serialize(
            &self.block_type_registry_checksum,
            stream,
        )?;
        <Uuid as ::bedrockrs_proto_core::ProtoCodec>::proto_serialize(
            &self.world_template_id,
            stream,
        )?;
        <bool as ::bedrockrs_proto_core::ProtoCodec>::proto_serialize(
            &self.enable_clientside_world_generation,
            stream,
        )?;
        <bool as ::bedrockrs_proto_core::ProtoCodec>::proto_serialize(
            &self.use_block_network_id_hashes,
            stream,
        )?;
        <NetworkPermissions as ::bedrockrs_proto_core::ProtoCodec>::proto_serialize(
            &self.network_permission,
            stream,
        )?;
        Ok(())
    }
    fn proto_deserialize(
        stream: &mut ::std::io::Cursor<&[u8]>,
    ) -> Result<Self, ::bedrockrs_proto_core::error::ProtoCodecError>
    where
        Self: Sized,
    {
        {
            let lvl = ::log::Level::Trace;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api::log(
                    format_args!("ProtoDeserialize: {0}", "StartGamePacket"),
                    lvl,
                    &(
                        "bedrockrs_proto::packets::start_game",
                        "bedrockrs_proto::packets::start_game",
                        ::log::__private_api::loc(),
                    ),
                    (),
                );
            }
        };
        let target_actor_id: ActorUniqueID = <ActorUniqueID as ::bedrockrs_proto_core::ProtoCodec>::proto_deserialize(
            stream,
        )?;
        let target_runtime_id: ActorRuntimeID = <ActorRuntimeID as ::bedrockrs_proto_core::ProtoCodec>::proto_deserialize(
            stream,
        )?;
        let gamemode: Gamemode = <Gamemode as ::bedrockrs_proto_core::ProtoCodec>::proto_deserialize(
            stream,
        )?;
        let position: Vec3<f32> = <Vec3<
            f32,
        > as ::bedrockrs_proto_core::ProtoCodecLE>::proto_deserialize(stream)?;
        let rotation: Vec2<f32> = <Vec2<
            f32,
        > as ::bedrockrs_proto_core::ProtoCodecLE>::proto_deserialize(stream)?;
        let settings: LevelSettings = <LevelSettings as ::bedrockrs_proto_core::ProtoCodec>::proto_deserialize(
            stream,
        )?;
        let level_id: String = <String as ::bedrockrs_proto_core::ProtoCodec>::proto_deserialize(
            stream,
        )?;
        let level_name: String = <String as ::bedrockrs_proto_core::ProtoCodec>::proto_deserialize(
            stream,
        )?;
        let template_content_identity: String = <String as ::bedrockrs_proto_core::ProtoCodec>::proto_deserialize(
            stream,
        )?;
        let trial: bool = <bool as ::bedrockrs_proto_core::ProtoCodec>::proto_deserialize(
            stream,
        )?;
        let movement_settings: PlayerMovementSettings = <PlayerMovementSettings as ::bedrockrs_proto_core::ProtoCodec>::proto_deserialize(
            stream,
        )?;
        let current_level_time: u64 = <u64 as ::bedrockrs_proto_core::ProtoCodecLE>::proto_deserialize(
            stream,
        )?;
        let enchantment_seed: i32 = <i32 as ::bedrockrs_proto_core::ProtoCodecVAR>::proto_deserialize(
            stream,
        )?;
        let blocks = {
            let len: u32 = <u32 as ::bedrockrs_proto_core::ProtoCodecVAR>::proto_deserialize(
                stream,
            )?;
            let mut vec = Vec::with_capacity(len.try_into()?);
            for _ in 0..len {
                vec.push(
                    <BlockEntry as ::bedrockrs_proto_core::ProtoCodec>::proto_deserialize(
                        stream,
                    )?,
                );
            }
            vec
        };
        let items = {
            let len: u32 = <u32 as ::bedrockrs_proto_core::ProtoCodecVAR>::proto_deserialize(
                stream,
            )?;
            let mut vec = Vec::with_capacity(len.try_into()?);
            for _ in 0..len {
                vec.push(
                    <ItemEntry as ::bedrockrs_proto_core::ProtoCodec>::proto_deserialize(
                        stream,
                    )?,
                );
            }
            vec
        };
        let multiplayer_correlation_id: String = <String as ::bedrockrs_proto_core::ProtoCodec>::proto_deserialize(
            stream,
        )?;
        let enable_item_stack_net_manager: bool = <bool as ::bedrockrs_proto_core::ProtoCodec>::proto_deserialize(
            stream,
        )?;
        let server_version: String = <String as ::bedrockrs_proto_core::ProtoCodec>::proto_deserialize(
            stream,
        )?;
        let player_property_data: nbtx::Value = ::nbtx::from_bytes::<
            ::nbtx::NetworkLittleEndian,
            _,
        >(stream)?;
        let block_type_registry_checksum: u64 = <u64 as ::bedrockrs_proto_core::ProtoCodecLE>::proto_deserialize(
            stream,
        )?;
        let world_template_id: Uuid = <Uuid as ::bedrockrs_proto_core::ProtoCodec>::proto_deserialize(
            stream,
        )?;
        let enable_clientside_world_generation: bool = <bool as ::bedrockrs_proto_core::ProtoCodec>::proto_deserialize(
            stream,
        )?;
        let use_block_network_id_hashes: bool = <bool as ::bedrockrs_proto_core::ProtoCodec>::proto_deserialize(
            stream,
        )?;
        let network_permission: NetworkPermissions = <NetworkPermissions as ::bedrockrs_proto_core::ProtoCodec>::proto_deserialize(
            stream,
        )?;
        let val = Self {
            target_actor_id,
            target_runtime_id,
            gamemode,
            position,
            rotation,
            settings,
            level_id,
            level_name,
            template_content_identity,
            trial,
            movement_settings,
            current_level_time,
            enchantment_seed,
            blocks,
            items,
            multiplayer_correlation_id,
            enable_item_stack_net_manager,
            server_version,
            player_property_data,
            block_type_registry_checksum,
            world_template_id,
            enable_clientside_world_generation,
            use_block_network_id_hashes,
            network_permission,
        };
        Ok(val)
    }
    fn get_size_prediction(&self) -> usize {
        1
    }
}
