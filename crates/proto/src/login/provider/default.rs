use crate::compression::Compression;
use crate::login::provider::packs::LoginProviderPacks;
use crate::login::provider::{LoginProviderServer, LoginProviderStatus};
use crate::packets::client_cache_status::ClientCacheStatusPacket;
use crate::packets::login::LoginPacket;
use crate::packets::network_settings::NetworkSettingsPacket;
use crate::packets::network_settings_request::NetworkSettingsRequestPacket;
use crate::packets::play_status::PlayStatusPacket;
use crate::packets::resource_packs_info::ResourcePacksInfoPacket;
use crate::packets::resource_packs_response::ResourcePacksResponsePacket;
use crate::packets::resource_packs_stack::ResourcePacksStackPacket;

pub struct DefaultLoginProvider {
    packs: LoginProviderPacks,
}

impl DefaultLoginProvider {
    pub fn new() -> Self {
        Self {
            packs: LoginProviderPacks::CDN {
                behavior_packs: vec![],
                resource_packs: vec![],
                cdn_urls: vec![],
            },
        }
    }
}

impl Default for DefaultLoginProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl LoginProviderServer for DefaultLoginProvider {
    fn compression(&self) -> Compression {
        Compression::Snappy { threshold: 1 }
    }
    fn encryption_enabled(&self) -> bool {
        false
    }

    fn auth_enabled(&self) -> bool {
        false
    }

    fn packs(&self) -> &LoginProviderPacks {
        &self.packs
    }

    fn on_network_settings_request_pk(
        &mut self,
        pk: &mut NetworkSettingsRequestPacket,
    ) -> LoginProviderStatus {
        LoginProviderStatus::ContinueLogin
    }

    fn on_network_settings_pk(&mut self, pk: &mut NetworkSettingsPacket) -> LoginProviderStatus {
        LoginProviderStatus::ContinueLogin
    }

    fn on_login_pk(&mut self, pk: &mut LoginPacket) -> LoginProviderStatus {
        LoginProviderStatus::ContinueLogin
    }

    fn on_play_status_pk(&mut self, pk: &mut PlayStatusPacket) -> LoginProviderStatus {
        LoginProviderStatus::ContinueLogin
    }
    fn on_resource_packs_info_pk(
        &mut self,
        pk: &mut ResourcePacksInfoPacket,
    ) -> LoginProviderStatus {
        LoginProviderStatus::ContinueLogin
    }
    fn on_resource_packs_stack_pk(
        &mut self,
        pk: &mut ResourcePacksStackPacket,
    ) -> LoginProviderStatus {
        LoginProviderStatus::ContinueLogin
    }
    fn on_resource_packs_response_pk(
        &mut self,
        pk: &mut ResourcePacksResponsePacket,
    ) -> LoginProviderStatus {
        LoginProviderStatus::ContinueLogin
    }
    fn on_client_cache_status_pk(&self, pk: &mut ClientCacheStatusPacket) -> LoginProviderStatus {
        LoginProviderStatus::ContinueLogin
    }
}
