use crate::compression::Compression;
use crate::login::provider::packs::LoginProviderPacks;
use crate::login::provider::status::LoginProviderStatus;
use crate::packets::client_cache_status::ClientCacheStatusPacket;
use crate::packets::login::LoginPacket;
use crate::packets::network_settings::NetworkSettingsPacket;
use crate::packets::network_settings_request::NetworkSettingsRequestPacket;
use crate::packets::play_status::PlayStatusPacket;
use crate::packets::resource_packs_info::ResourcePacksInfoPacket;
use crate::packets::resource_packs_response::ResourcePacksResponsePacket;
use crate::packets::resource_packs_stack::ResourcePacksStackPacket;

pub trait LoginProvider {
    fn compression(&self) -> Compression;
    fn encryption_enabled(&self) -> bool;
    fn auth_enabled(&self) -> bool;

    fn packs(&self) -> &LoginProviderPacks;

    fn on_network_settings_request_pk(
        &mut self,
        _pk: &mut NetworkSettingsRequestPacket,
    ) -> LoginProviderStatus {
        LoginProviderStatus::ContinueLogin
    }
    
    fn on_network_settings_pk(&mut self, _pk: &mut NetworkSettingsPacket) -> LoginProviderStatus {
        LoginProviderStatus::ContinueLogin
    }
    
    fn on_login_pk(&mut self, _pk: &mut LoginPacket) -> LoginProviderStatus {
        LoginProviderStatus::ContinueLogin
    }
    
    fn on_play_status_pk(&mut self, _pk: &mut PlayStatusPacket) -> LoginProviderStatus {
        LoginProviderStatus::ContinueLogin
    }
    
    fn on_resource_packs_info_pk(
        &mut self,
        _pk: &mut ResourcePacksInfoPacket,
    ) -> LoginProviderStatus {
        LoginProviderStatus::ContinueLogin
    }
    
    fn on_resource_packs_stack_pk(
        &mut self,
        _pk: &mut ResourcePacksStackPacket,
    ) -> LoginProviderStatus {
        LoginProviderStatus::ContinueLogin
    }
    
    fn on_resource_packs_response_pk(
        &mut self,
        _pk: &mut ResourcePacksResponsePacket,
    ) -> LoginProviderStatus {
        LoginProviderStatus::ContinueLogin
    }
    
    fn on_client_cache_status_pk(&self, _pk: &mut ClientCacheStatusPacket) -> LoginProviderStatus {
        LoginProviderStatus::ContinueLogin
    }
}
