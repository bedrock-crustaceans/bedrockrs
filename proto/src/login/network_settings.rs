use bedrockrs_core::LE;

use crate::connection::ConnectionShard;
use crate::error::LoginError;
use crate::gamepacket::GamePacket;
use crate::login::provider::{LoginProviderServer, LoginProviderStatus};
use crate::packets::network_settings::NetworkSettingsPacket;

pub async fn network_settings(
    conn: &mut ConnectionShard,
    provider: &mut impl LoginProviderServer,
) -> Result<(), LoginError> {
    //////////////////////////////////////
    // Network Settings Request Packet
    //////////////////////////////////////

    let mut network_settings_request = match conn.recv().await {
        Ok(GamePacket::RequestNetworkSettings(pk)) => pk,
        Ok(other) => {
            return Err(LoginError::FormatError(format!(
                "Expected RequestNetworkSettings packet, got: {other:?}"
            )))
        }
        Err(e) => return Err(LoginError::ConnectionError(e)),
    };

    match provider.on_network_settings_request_pk(&mut network_settings_request) {
        LoginProviderStatus::ContinueLogin => {}
        LoginProviderStatus::AbortLogin { reason } => {
            return Err(LoginError::Abort { reason });
        }
    };

    //////////////////////////////////////
    // Network Settings Packet
    //////////////////////////////////////

    let compression = provider.compression();

    let mut network_settings = NetworkSettingsPacket {
        compression_threshold: LE::new(compression.threshold()),
        compression_algorithm: LE::new(compression.id_u16()),
        // TODO What do these 3 fields do?
        client_throttle_enabled: false,
        client_throttle_threshold: 0,
        client_throttle_scalar: LE::new(0.0),
    };

    match provider.on_network_settings_pk(&mut network_settings) {
        LoginProviderStatus::ContinueLogin => {}
        LoginProviderStatus::AbortLogin { reason } => {
            return Err(LoginError::Abort { reason });
        }
    };

    match conn
        .send(GamePacket::NetworkSettings(network_settings))
        .await
    {
        Ok(_) => {}
        Err(e) => return Err(LoginError::ConnectionError(e)),
    }

    match conn.flush().await {
        Ok(_) => {}
        Err(e) => return Err(LoginError::ConnectionError(e)),
    }

    match conn.set_compression(Some(compression)).await {
        Ok(_) => {}
        Err(e) => return Err(LoginError::ConnectionError(e)),
    };

    Ok(())
}
