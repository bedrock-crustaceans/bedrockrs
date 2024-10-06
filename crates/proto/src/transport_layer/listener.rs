use crate::error::{RaknetError, TransportLayerError};
use crate::transport_layer::TransportLayerConnection;

pub enum TransportLayerListener {
    RakNet(rak_rs::Listener),
    // TODO NetherNet(...),
    // TODO Quic(...),
    // TODO Tcp(...),
}

impl TransportLayerListener {
    pub async fn start(&mut self) -> Result<(), TransportLayerError> {
        match self {
            TransportLayerListener::RakNet(listener) => listener.start().await.map_err(|err| TransportLayerError::RakNetError(RaknetError::ServerError(err)))?,
        };
        
        Ok(())
    }
    
    pub async fn stop(&mut self) -> Result<(), TransportLayerError> {
        match self {
            TransportLayerListener::RakNet(listener) => listener.stop().await.map_err(|err| TransportLayerError::RakNetError(RaknetError::ServerError(err)))?,
        }
        
        Ok(())
    }

    pub async fn accept(&mut self) -> Result<TransportLayerConnection, TransportLayerError> {
        let conn = match self {
            TransportLayerListener::RakNet(listener) => TransportLayerConnection::RakNet(listener.accept().await.map_err(|err| TransportLayerError::RakNetError(RaknetError::ServerError(err)))?),
        };
        
        Ok(conn)
    }
}
