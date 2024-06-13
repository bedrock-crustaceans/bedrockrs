use bedrock_core::LE;

use crate::conn::Connection;
use crate::error::LoginError;
use crate::gamepacket::GamePacket;
use crate::login::provider::LoginProviderStatus;
use crate::login::provider::{LoginProviderClient, LoginProviderServer};
use crate::packets::network_settings::NetworkSettingsPacket;

macro_rules! recv_single_packet {
    ($conn:ident, $pk:ident, $pk_name:expr, $provider:ident, $handler:ident) => {{
        // Receive a packet from the connection
        let packet = match $conn.recv().await {
            // Extract the first packet from the vector
            Ok(mut vec) => match vec.into_iter().next() {
                None => {
                    return Err(LoginError::FormatError(format!(
                        "Recieved empty gamepacket vec, at {}",
                        $pk_name
                    )))
                }
                Some(v) => v,
            },
            Err(e) => return Err(LoginError::ConnError(e)),
        };

        // Match the received packet to the expected packet type
        match packet {
            GamePacket::$pk(mut packet) => {
                // Handle the packet using the provided handler
                match $provider.$handler(&mut packet) {
                    LoginProviderStatus::ContinueLogin => packet,
                    LoginProviderStatus::AbortLogin { reason } => {
                        return Err(LoginError::Abort { reason });
                    }
                }
            }
            other => {
                return Err(LoginError::FormatError(format!(
                    "Expected {}, got {:?}",
                    $pk_name, other
                )))
            }
        }
    }};
}

macro_rules! send_single_packet {
    ($conn:ident, $pk:ident, $pk_type:ident, $pk_name:expr, $provider:ident, $handler:ident) => {{
        // Handle the packet using the provided handler
        match $provider.$handler(&mut $pk) {
            LoginProviderStatus::ContinueLogin => {}
            LoginProviderStatus::AbortLogin { reason } => {
                return Err(LoginError::Abort { reason });
            }
        };

        // Send the packet via the connection
        match $conn.send(vec![GamePacket::$pk_type($pk)]).await {
            Ok(_) => {}
            Err(e) => return Err(LoginError::ConnError(e)),
        }
    }};
}

pub async fn login_to_server(
    conn: &mut Connection,
    provider: impl LoginProviderServer,
) -> Result<(), LoginError> {
    let _network_settings_request = recv_single_packet!(
        conn,
        RequestNetworkSettings,
        "RequestNetworkSettings",
        provider,
        on_network_settings_request_pk
    );

    let compression = provider.compression();

    let mut network_settings = NetworkSettingsPacket {
        compression_threshold: LE::new(0),
        compression_algorithm: LE::new(0xFFFF),
        // TODO What do these 3 fields do?
        client_throttle_enabled: false,
        client_throttle_threshold: LE::new(0),
        client_throttle_scalar: LE::new(0.0),
    };

    send_single_packet!(
        conn,
        network_settings,
        NetworkSettings,
        "NetworkSettings",
        provider,
        on_network_settings_pk
    );

    // conn.send_raw(&[12, 143, 1, 0, 0, 255, 255, 0, 0, 0, 0, 0, 0]).await.unwrap();

    conn.compression = Some(compression);

    let _login = recv_single_packet!(conn, Login, "Login", provider, on_login_pk);

    Ok(())
}

pub async fn login_to_client(
    conn: &mut Connection,
    provider: impl LoginProviderClient,
) -> Result<(), LoginError> {
    Ok(())
}
