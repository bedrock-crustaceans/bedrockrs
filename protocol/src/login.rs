use std::future::Future;
use bedrock_core::types::u16le;

use crate::compression::{CompressionMethod, CompressionMethods};
use crate::compression::none::NoCompression;
use crate::compression::snappy::SnappyCompression;
use crate::compression::zlib::ZlibCompression;
use crate::conn::Connection;
use crate::error::{ConnectionError, LoginError};
use crate::gamepacket::GamePacket;
use crate::packets::network_settings::NetworkSettingsPacket;
use crate::packets::play_status::PlayStatusPacket;
use crate::packets::resource_packs_info::ResourcePacksInfoPacket;
use crate::types::play_status::PlayStatusType;

pub struct LoginServerSideOptions {
    pub compression: CompressionMethods,
    pub encryption: bool,
    /// TODO: impl Microsoft Auth
    pub authentication_enabled: bool,
    /// Even if the client has another protocol version the login
    /// process should continue, might cause problems when the login process
    /// differs in the different protocol versions
    pub allowed_proto_versions: Vec<i32>,
}

pub async fn handle_login_server_side(
    connection: &mut Connection,
    options: LoginServerSideOptions,
) -> Result<(), LoginError> {
    // Receive NetworkRequestSettings
    let gamepackets = match connection.recv_gamepackets().await {
        Ok(v) => v,
        Err(_) => return Err(LoginError::PacketMissmatch),
    };

    // If too many or no packets were send then error
    if gamepackets.len() > 1 || gamepackets.len() < 1 {
        return Err(LoginError::PacketMissmatch);
    }

    // Get the clients protocol version
    let client_proto_ver = match gamepackets[0] {
        GamePacket::RequestNetworkSettings(pk) => pk.client_network_version.0,
        _ => return Err(LoginError::PacketMissmatch),
    };

    // Look if Protocol versions match and if other protocol versions are allowed
    if !options.allowed_proto_versions.contains(&client_proto_ver) {
        return Err(LoginError::WrongProtocolVersion {
            client: client_proto_ver,
        });
    }

    println!("NETWORK SETTINGS REQUEST");

    // Get the compression threshold and id for the network settings packet
    let (threshold, id_u16) = match &options.compression {
        CompressionMethods::Zlib(zlib) => (zlib.threshold, ZlibCompression::ID_u16),
        CompressionMethods::Snappy(snappy) => (snappy.threshold, SnappyCompression::ID_u16),
        CompressionMethods::None => (0, NoCompression::ID_u16),
    };

    // Send Network Settings
    match connection
        .send_gamepackets(vec![GamePacket::NetworkSettings(NetworkSettingsPacket {
            compression_threshold: u16le(threshold),
            compression_algorythm: u16le(id_u16),
            // TODO: Figure out what all of this is
            client_throttle_enabled: false,
            client_throttle_threshold: 0,
            client_throttle_scalar: 0.0,
        })])
        .await
    {
        Ok(_) => {}
        Err(e) => return Err(LoginError::ConnError(e)),
    };

    println!("NETWORK SETTINGS");

    // Enable Compression after Network Settings
    connection.set_compression_method(Some(options.compression));

    // Receive Login
    let login = match connection.recv_gamepackets().await {
        Ok(v) => v,
        Err(e) => {
            println!("{:?}", e);
            return Err(LoginError::PacketMissmatch);
        }
    };

    // If too many or no packets were send the error
    if login.len() > 1 || login.len() < 1 {
        return Err(LoginError::PacketMissmatch);
    }

    // Get the clients protocol version
    let login_pk = match &login[0] {
        GamePacket::Login(pk) => pk,
        _ => return Err(LoginError::PacketMissmatch),
    };

    // If encryption is enabled send the
    if options.encryption {
        // TODO: Setup Encryption
    }

    // Sent PlayStatus Login successful packet to indicate that the login was successful
    match connection.send_gamepackets(vec![GamePacket::PlayStatus(PlayStatusPacket {
        status: PlayStatusType::LoginSuccess
    })]).await {
        Ok(_) => {}
        Err(e) => { return Err(LoginError::ConnError(e)) }
    };

    println!("PLAY STATUS LOGIN");

    // Sent the Resource Packs Info packet with the needed data
    // TODO: Make this useful
    match connection.send_gamepackets(vec![GamePacket::ResourcePacksInfo(ResourcePacksInfoPacket{
        resource_pack_required: false,
        has_addon_packs: false,
        has_scripts: false,
        force_server_packs_enabled: false,
        behavior_packs: vec![],
        resource_packs: vec![],
        cdn_urls: vec![],
    })]).await {
        Ok(_) => {},
        Err(e) => { return Err(LoginError::ConnError(e)) },
    }

    // Receive Client Cache Status
    let cache_status_pk = match connection.recv_gamepackets().await {
        Ok(v) => { v }
        Err(e) => { return Err(LoginError::ConnError(e)) }
    };

    // Receive Resource Packs Response
    match connection.recv_gamepackets().await {
        Ok(v) => { println!("PK: {v:#?}") }
        Err(e) => { return Err(LoginError::ConnError(e)) }
    }

    Ok(())
}

pub fn handle_login_client_side(connection: &mut Connection) {
    todo!()
}
