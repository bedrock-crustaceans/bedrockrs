use crate::error::{RaknetError, TransportLayerError};
use crate::transport_layer::TransportLayerConnection;

pub enum TransportLayerListener {
    RaknetUDP(rak_rs::Listener),
    // TODO NetherNet(/* TODO */),
}

impl TransportLayerListener {
    pub async fn start(&mut self) -> Result<(), TransportLayerError> {
        match self {
            TransportLayerListener::RaknetUDP(listener) => listener.start().await.map_err(|err| TransportLayerError::RaknetUDPError(RaknetError::ServerError(err)))?,
        };
        
        Ok(())
    }
    
    pub async fn stop(&mut self) -> Result<(), TransportLayerError> {
        match self {
            TransportLayerListener::RaknetUDP(listener) => listener.stop().await.map_err(|err| TransportLayerError::RaknetUDPError(RaknetError::ServerError(err)))?,
        }
        
        Ok(())
    }

    pub async fn accept(&mut self) -> Result<TransportLayerConnection, TransportLayerError> {
        let conn = match self {
            TransportLayerListener::RaknetUDP(listener) => TransportLayerConnection::RaknetUDP(listener.accept().await.map_err(|err| TransportLayerError::RaknetUDPError(RaknetError::ServerError(err)))?),
        };
        
        Ok(conn)
    }
}
