use io::Error as IOError;
use std::error::Error;
use std::io;

use bedrockrs_proto_core::error::ProtoCodecError;
use rak_rs::connection::queue::SendQueueError;
use rak_rs::connection::RecvError;
use rak_rs::error::server::ServerError;
use thiserror::Error;

use crate::info::RAKNET_GAMEPACKET_ID;

#[derive(Error, Debug)]
pub enum ListenerError {
    #[error("Address bind error")]
    AddrBindError,
    #[error("Already Online")]
    AlreadyOnline,
    #[error("Not Listening")]
    NotListening,
    #[error("Transport Error: {0}")]
    TransportListenerError(#[from] TransportLayerError),
}

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("Proto Codec Error: {0}")]
    ProtoCodecError(#[from] ProtoCodecError),
    #[error("Connection Closed")]
    ConnectionClosed,
    #[error("Transport Error: {0}")]
    TransportError(#[from] TransportLayerError),
    #[error("IO Error: {0}")]
    IOError(#[from] IOError),
}

#[derive(Error, Debug)]
pub enum TransportLayerError {
    #[error("IO Error: {0}")]
    IOError(#[from] IOError),
    #[error("Raknet UDP Error: {0}")]
    RakNetError(#[from] RaknetError),
}

#[derive(Error, Debug, Clone)]
pub enum RaknetError {
    #[error("Error while Receive: {0}")]
    RecvError(#[from] RecvError),
    #[error("Error while Send: {0}")]
    SendError(#[from] SendQueueError),
    #[error("Server Error: {0}")]
    ServerError(#[from] ServerError),
    #[error("Invalid RakNet Header, expected: {RAKNET_GAMEPACKET_ID}, got: {0}")]
    InvalidRakNetHeader(u8),
    #[error("Format Error: {0}")]
    FormatError(String),
}
