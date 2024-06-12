use bedrock_core::LE;
use crate::conn::Connection;
use crate::error::{ConnectionError, LoginError};
use crate::gamepacket::GamePacket;
use crate::gamepacket::GamePacket::RequestNetworkSettings;
use crate::login::provider::{LoginProviderClient, LoginProviderServer};
use crate::packets::network_settings::NetworkSettingsPacket;

macro_rules! recv_single_packet {
    ($conn:ident, $pk:ident, $pk_name:expr, $provider:ident, $handler:ident) => ({
        let packet = match $conn.recv().await {
            Ok(mut vec) => { match vec.into_iter().nth(0) {
                None => { return Err(LoginError::FormatError(format!("Recieved empty gamepacket vec, at login: {}", $pk_name))) }
                Some(v) => { v }
            } }
            Err(e) => { return Err(LoginError::ConnError(e)) }
        };

        match packet {
            GamePacket::$pk(mut pk) => {
                $provider.$handler(&mut pk);
                pk
            }
            _ => { return Err(LoginError::FormatError(format!(""))) }
        }
    });
}

macro_rules! send_single_packet {
    ($conn:ident, $pk:ident, $handler:ident) => ({
        match $conn.send(vec![$pk]).await {
            Ok(_) => { }
            Err(e) => { return Err(LoginError::ConnError(e)) }
        };
    });
}

pub async fn login_to_server(conn: &mut Connection, provider: impl LoginProviderServer) -> Result<(), LoginError> {
    let network_settings_request = recv_single_packet!(conn, RequestNetworkSettings, "RequestNetworkSettings", provider, on_network_settings_request_pk);


    let network_settings = GamePacket::NetworkSettings(NetworkSettingsPacket{
        compression_threshold: LE::new(provider.compression().threshold()),
        compression_algorithm: LE::new(provider.compression().id_u16()),
        // TODO What do these 3 fields do?
        client_throttle_enabled: false,
        client_throttle_threshold: LE::new(0),
        client_throttle_scalar: LE::new(0.0),
    });

    //send_single_packet!(conn, network_settings, provider.on_network_settings_pk)

    Ok(())
}

pub async fn login_to_client(conn: &mut Connection, provider: impl LoginProviderClient) -> Result<(), LoginError> {
    Ok(())
}
