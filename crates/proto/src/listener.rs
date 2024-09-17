use core::net::SocketAddr;

use rak_rs::mcpe::motd::Gamemode;
use rak_rs::Motd;
use rand::RngCore;

use crate::connection::Connection;
use crate::error::{ListenerError, RaknetError, TransportLayerError};
use crate::info::{MINECRAFT_EDITION_MOTD, MINECRAFT_VERSION, PROTOCOL_VERSION};
use crate::transport_layer::TransportLaterListener;

pub struct Listener {
    listener: TransportLaterListener,
    name: String,
    sub_name: String,
    player_max: u32,
    player_count: u32,
    socket_addr: SocketAddr,
    guid: u64,
}

impl Listener {
    pub async fn new_raknet(
        name: String,
        sub_name: String,
        display_version: String,
        player_max: u32,
        player_count: u32,
        socket_addr: SocketAddr,
        nintendo_limited: bool,
    ) -> Result<Self, ListenerError> {
        // Bind the Raknet Listener
        let rak_listener = rak_rs::Listener::bind(socket_addr).await;

        // Check for success
        let mut rak_listener = match rak_listener {
            Ok(v) => v,
            Err(e) => {
                return Err(ListenerError::TransportListenerError(
                    TransportLayerError::RaknetUDPError(RaknetError::ServerError(e)),
                ));
            }
        };

        // generate a random guid
        let guid: u64 = rand::thread_rng().next_u64();

        // Setup the motd
        rak_listener.motd = Motd {
            edition: String::from(MINECRAFT_EDITION_MOTD),
            version: display_version,
            name: name.clone(),
            sub_name: sub_name.clone(),
            player_max,
            player_count,
            protocol: PROTOCOL_VERSION as u16,
            server_guid: guid,
            gamemode: Gamemode::Survival,
            port: Some(socket_addr.clone().port().to_string()),
            ipv6_port: Some(socket_addr.clone().port().to_string()),
            nintendo_limited: Some(nintendo_limited),
        };

        Ok(Self {
            listener: TransportLaterListener::RaknetUDP(rak_listener),
            name,
            sub_name,
            player_max,
            player_count,
            socket_addr,
            guid,
        })
    }

    pub async fn start(&mut self) -> Result<(), ListenerError> {
        match self.listener.start().await {
            Ok(_) => Ok(()),
            Err(e) => Err(ListenerError::TransportListenerError(e)),
        }
    }

    pub async fn accept(&mut self) -> Result<Connection, ListenerError> {
        let rak_conn = match self.listener.accept().await {
            Ok(c) => c,
            Err(e) => return Err(ListenerError::TransportListenerError(e)),
        };

        Ok(Connection::from_transport_conn(rak_conn))
    }
}
