use crate::connection::ConnectionShard;
use crate::error::LoginError;
use crate::gamepackets::GamePackets;
use crate::login::provider::{LoginProvider, LoginProviderStatus};
use crate::packets::network_settings::NetworkSettingsPacket;

pub async fn network_settings(
    conn: &mut ConnectionShard,
    provider: &mut impl LoginProvider,
) -> LoginProviderStatus {
    //////////////////////////////////////
    // Network Settings Request Packet
    //////////////////////////////////////

    let mut network_settings_request = match conn.recv().await? {
        GamePackets::NetworkSettingsRequest(pk) => pk,
        other => {
            return Err(LoginError::FormatError(format!(
                "Expected RequestNetworkSettings packet, got: {other:?}"
            )))
        }
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
        compression_threshold: compression.threshold(),
        compression_algorithm: compression.id_u16(),
        // TODO What do these 3 fields do?
        client_throttle_enabled: false,
        client_throttle_threshold: 0,
        client_throttle_scalar: 0.0,
    };

    match provider.on_network_settings_pk(&mut network_settings) {
        LoginProviderStatus::ContinueLogin => {}
        LoginProviderStatus::AbortLogin { reason } => {
            return Err(LoginError::Abort { reason });
        }
    };

    conn.send(GamePackets::NetworkSettings(network_settings))
        .await?;
    conn.flush().await?;

    conn.set_compression(Some(compression)).await?;

    Ok(())
}
