use crate::compression::Compression;
use crate::login::provider::status::LoginProviderStatus;
use crate::packets::login::LoginPacket;
use crate::packets::network_settings::NetworkSettingsPacket;
use crate::packets::network_settings_request::NetworkSettingsRequestPacket;

pub trait LoginProviderServer {
    fn compression(&self) -> Compression;
    fn encryption_enabled(&self) -> bool;

    fn on_network_settings_request_pk(
        &self,
        pk: &mut NetworkSettingsRequestPacket,
    ) -> LoginProviderStatus {
        LoginProviderStatus::ContinueLogin
    }

    fn on_network_settings_pk(&self, pk: &mut NetworkSettingsPacket) -> LoginProviderStatus {
        LoginProviderStatus::ContinueLogin
    }

    fn on_login_pk(&self, pk: &mut LoginPacket) -> LoginProviderStatus {
        LoginProviderStatus::ContinueLogin
    }
}

pub trait LoginProviderClient {}
