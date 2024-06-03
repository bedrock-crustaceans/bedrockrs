use core::net::SocketAddr;
use core::net::SocketAddrV4;

use rak_rs::mcpe::motd::Gamemode;
use rak_rs::Motd;
use rand::RngCore;

use crate::conn::Conn;
use crate::error::{ListenerError, RaknetError, TransportLayerError};
use crate::info::{MINECRAFT_EDITION_MOTD, MINECRAFT_VERSION, PROTOCOL_VERSION};
use crate::transport_layer::{TransportLaterListener, TransportLayerType};

pub struct Listener {
    listener: TransportLaterListener,
    pub config: ListenerConfig,
    socket_addr_v4: SocketAddrV4,
    guid: u64,
}

impl Listener {
    pub async fn new_raknet(
        listener_config: ListenerConfig,
        socket_addr_v4: SocketAddrV4,
    ) -> Result<Self, ListenerError> {
        // Bind the Raknet Listener
        let rak_listener = rak_rs::Listener::bind(SocketAddr::V4(socket_addr_v4.clone())).await;

        // Check for success
        let mut rak_listener = match rak_listener {
            Ok(v) => v,
            Err(e) => return Err(ListenerError::TransportListenerError(TransportLayerError::RaknetUDPError(RaknetError::ServerError(e)))),
        };

        // generate a random guid
        let guid: u64 = rand::thread_rng().next_u64();

        // Setup the motd
        rak_listener.motd = Motd {
            edition: String::from(MINECRAFT_EDITION_MOTD),
            // Prevent String no impl copy
            name: listener_config.name.as_str().to_string(),
            sub_name: listener_config.sub_name.as_str().to_string(),
            protocol: PROTOCOL_VERSION as u16,
            version: String::from(MINECRAFT_VERSION),
            server_guid: guid,
            gamemode: Gamemode::Survival,
            port: Some(socket_addr_v4.clone().port().to_string()),
            // TODO: Add this when ipv6 is supported by rak-rs
            ipv6_port: Some(socket_addr_v4.clone().port().to_string()),
            player_max: listener_config.player_count_max,
            player_count: listener_config.player_count_current,
            nintendo_limited: Some(listener_config.nintendo_limited),
        };

        Ok(Self {
            listener: TransportLaterListener::RaknetUDP(rak_listener),
            config: listener_config,
            socket_addr_v4,
            guid,
        })
    }

    pub async fn start(&mut self) -> Result<(), ListenerError> {
        match self.listener.start().await {
            Ok(_) => Ok(()),
            Err(e) => Err(ListenerError::TransportListenerError(e)),
        }
    }

    pub async fn accept(&mut self) -> Result<Conn, ListenerError> {
        let rak_conn = match self.listener.accept().await {
            Ok(c) => c,
            Err(e) => return Err(ListenerError::TransportListenerError(e)),
        };

        Ok(Conn::new(rak_conn))
    }

    fn update_pongdata() {}
}

#[derive(Debug, Eq, PartialEq)]
pub struct ListenerConfig {
    pub name: String,
    pub sub_name: String,
    pub player_count_max: u32,
    pub player_count_current: u32,
    pub nintendo_limited: bool,
}
