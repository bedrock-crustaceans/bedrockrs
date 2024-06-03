use std::future::Future;
use rak_rs::connection::Connection;
use rak_rs::error::server::ServerError;
use crate::error::{RaknetError, TransportLayerError};
use crate::transport_layer::TransportLayerConn;

pub enum TransportLaterListener {
    RaknetUDP(rak_rs::Listener),
    NetherNet(/* TODO */)
}

impl TransportLaterListener {
    pub async fn start(&mut self) -> Result<(), TransportLayerError> {
        match self {
            TransportLaterListener::RaknetUDP(listener) => {
                match listener.start().await {
                    Ok(_) => { Ok(()) }
                    Err(e) => { Err(TransportLayerError::RaknetUDPError(RaknetError::ServerError(e))) }
                }
            }
            _ => { todo!() }
        }
    }

    pub async fn accept(&mut self) -> Result<TransportLayerConn, TransportLayerError> {
        match self {
            TransportLaterListener::RaknetUDP(listener) => {
                match listener.accept().await {
                    Ok(conn) => { Ok(TransportLayerConn::RaknetUDP(conn)) }
                    Err(e) => { Err(TransportLayerError::RaknetUDPError(RaknetError::ServerError(e))) }
                }
            }
            _ => { todo!() }
        }
    }
}