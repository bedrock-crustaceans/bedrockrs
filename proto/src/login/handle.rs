use bedrock_core::LE;

use crate::connection::{Connection, ConnectionShard};
use crate::error::LoginError;
use crate::gamepacket::GamePacket;
use crate::login::provider::{LoginProviderClient, LoginProviderServer};
use crate::login::provider::LoginProviderStatus;
use crate::packets::network_settings::NetworkSettingsPacket;
use crate::packets::play_status::PlayStatusPacket;
use crate::types::play_status::PlayStatusType;

macro_rules! handle_packet {
    ($provider:ident, $handler:ident, $pk:ident) => {
        // Handle the packet using the provided handler
        match $provider.$handler(&mut $pk) {
            LoginProviderStatus::ContinueLogin => {}
            LoginProviderStatus::AbortLogin { reason } => {
                return Err(LoginError::Abort { reason });
            }
        };
    };
}

pub async fn login_to_server(
    conn: &mut ConnectionShard,
    provider: impl LoginProviderServer,
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
        Err(e) => return Err(LoginError::ConnError(e)),
    };

    handle_packet!(
        provider,
        on_network_settings_request_pk,
        network_settings_request
    );

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

    handle_packet!(provider, on_network_settings_pk, network_settings);

    match conn
        .send(GamePacket::NetworkSettings(network_settings))
        .await
    {
        Ok(_) => {}
        Err(e) => return Err(LoginError::ConnError(e)),
    }

    match conn.flush().await {
        Ok(_) => {}
        Err(e) => { return Err(LoginError::ConnError(e)) }
    }

    match conn.set_compression(Some(compression)).await {
        Ok(_) => {}
        Err(e) => { return Err(LoginError::ConnError(e)) }
    };

    //////////////////////////////////////
    // Login Packet
    //////////////////////////////////////

    let mut login = match conn.recv().await {
        Ok(GamePacket::Login(pk)) => pk,
        Ok(other) => {
            return Err(LoginError::FormatError(format!(
                "Expected Login packet, got: {other:?}"
            )))
        }
        Err(e) => return Err(LoginError::ConnError(e)),
    };

    handle_packet!(provider, on_login_pk, login);

    if provider.auth_enabled() {
        todo!("impl xbox auth with data from login pk")
    }

    //////////////////////////////////////
    // Play Status Packet
    //////////////////////////////////////

    let mut play_status = PlayStatusPacket {
        status: PlayStatusType::LoginSuccess,
    };

    handle_packet!(provider, on_play_status_pk, play_status);

    match conn
        .send(GamePacket::PlayStatus(play_status))
        .await
    {
        Ok(_) => {}
        Err(e) => return Err(LoginError::ConnError(e)),
    }

    match conn.flush().await {
        Ok(_) => {}
        Err(e) => { return Err(LoginError::ConnError(e)) }
    }

    Ok(())
}

pub async fn login_to_client(
    conn: &mut Connection,
    provider: impl LoginProviderClient,
) -> Result<(), LoginError> {
    todo!()
}
