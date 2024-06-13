use crate::compression::Compression;
use crate::login::provider::{LoginProviderServer, LoginProviderStatus};
use crate::packets::login::LoginPacket;
use crate::packets::network_settings::NetworkSettingsPacket;
use crate::packets::network_settings_request::NetworkSettingsRequestPacket;

pub struct DefaultLoginProvider {}

impl DefaultLoginProvider {
    pub fn new() -> Self {
        Self {}
    }
}

impl LoginProviderServer for DefaultLoginProvider {
    fn compression(&self) -> Compression {
        Compression::None
    }

    fn encryption_enabled(&self) -> bool {
        false
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
}
