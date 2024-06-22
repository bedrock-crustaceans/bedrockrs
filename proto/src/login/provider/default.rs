use crate::compression::Compression;
use crate::login::provider::{LoginProviderServer, LoginProviderStatus};
use crate::login::provider::packs::LoginProviderPacks;
use crate::packets::login::LoginPacket;
use crate::packets::network_settings::NetworkSettingsPacket;
use crate::packets::network_settings_request::NetworkSettingsRequestPacket;
use crate::packets::play_status::PlayStatusPacket;

pub struct DefaultLoginProvider {
    packs: LoginProviderPacks
}

impl DefaultLoginProvider {
    pub fn new() -> Self {
        Self {
            packs: LoginProviderPacks::CDN {
                behavior_packs: vec![],
                resource_packs: vec![]
            }
        }
    }
}

impl LoginProviderServer for DefaultLoginProvider {
    fn compression(&self) -> Compression {
        Compression::None
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
        &self,
        pk: &mut NetworkSettingsRequestPacket,
    ) -> LoginProviderStatus {
        println!("{:#?}", pk);
        LoginProviderStatus::ContinueLogin
    }

    fn on_network_settings_pk(&self, pk: &mut NetworkSettingsPacket) -> LoginProviderStatus {
        println!("{:#?}", pk);
        LoginProviderStatus::ContinueLogin
    }

    fn on_login_pk(&self, pk: &mut LoginPacket) -> LoginProviderStatus {
        println!("{:#?}", pk);
        LoginProviderStatus::ContinueLogin
    }

    fn on_play_status_pk(&self, pk: &mut PlayStatusPacket) -> LoginProviderStatus {
        println!("{:#?}", pk);
        LoginProviderStatus::ContinueLogin
    }
}
