use core::net::SocketAddr;

use rak_rs::mcpe::motd::Gamemode;
use rak_rs::Motd;
use rand::RngCore;

use crate::r#mod::Connection;
use crate::error::{ListenerError, RaknetError, TransportLayerError};
use crate::info::{MINECRAFT_EDITION_MOTD, PROTOCOL_VERSION};
use crate::transport_layer::TransportLayerListener;

pub struct Listener {
    listener: TransportLayerListener,
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
        let mut rak_listener = rak_rs::Listener::bind(socket_addr).await.map_err(|err| {
            ListenerError::TransportListenerError(TransportLayerError::RakNetError(
                RaknetError::ServerError(err),
            ))
        })?;

        // generate a random guid
        let guid: u64 = rand::thread_rng().next_u64();

        // Set up the motd
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
            listener: TransportLayerListener::RakNet(rak_listener),
            name,
            sub_name,
            player_max,
            player_count,
            socket_addr,
            guid,
        })
    }

    pub async fn start(&mut self) -> Result<(), ListenerError> {
        self.listener.start().await?;
        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), ListenerError> {
        self.listener.stop().await?;
        Ok(())
    }

    pub async fn accept(&mut self) -> Result<Connection, ListenerError> {
        let rak_conn = self.listener.accept().await?;

        Ok(Connection::from_transport_conn(rak_conn))
    }
}
