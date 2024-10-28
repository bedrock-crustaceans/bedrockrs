pub mod shard;

use crate::codec::{batch_gamepackets, separate_gamepackets};
use crate::compression::Compression;
use crate::encryption::Encryption;
use crate::error::ConnectionError;
use crate::helper::ProtoHelper;
use crate::transport_layer::TransportLayerConnection;

pub struct Connection {
    /// Represents the Connection's internal transport layer, which may vary
    transport_layer: TransportLayerConnection,
    /// Represents the Connection's Compression, the compression gets initialized in the
    /// login process
    pub compression: Option<Compression>,
    /// Represents the connections encryption, the encryption gets initialized in the
    /// login process, if encryption is enabled
    pub encryption: Option<Encryption>,
}

impl Connection {
    pub(crate) fn from_transport_conn(transport_layer: TransportLayerConnection) -> Self {
        Self {
            transport_layer,
            compression: None,
            encryption: None,
        }
    }

    pub async fn send<T: ProtoHelper>(
        &mut self,
        gamepackets: &[T::GamePacketType],
    ) -> Result<(), ConnectionError> {
        let gamepacket_stream =
            batch_gamepackets::<T>(gamepackets, &self.compression, &mut self.encryption)?;

        self.transport_layer.send(&gamepacket_stream).await?;

        Ok(())
    }

    pub async fn send_raw(&mut self, data: &[u8]) -> Result<(), ConnectionError> {
        self.transport_layer.send(data).await?;

        Ok(())
    }

    pub async fn recv<T: ProtoHelper>(
        &mut self,
    ) -> Result<Vec<T::GamePacketType>, ConnectionError> {
        let gamepacket_stream = self.transport_layer.recv().await?;

        let gamepackets =
            separate_gamepackets::<T>(gamepacket_stream, &self.compression, &mut self.encryption)?;

        Ok(gamepackets)
    }

    pub async fn recv_raw(&mut self) -> Result<Vec<u8>, ConnectionError> {
        let stream = self.transport_layer.recv().await?;

        Ok(stream)
    }

    pub async fn close(self) {
        self.transport_layer.close().await;
    }
}
