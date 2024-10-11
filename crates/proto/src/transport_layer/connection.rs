use crate::error::{RaknetError, TransportLayerError};
use crate::info::RAKNET_GAMEPACKET_ID;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Write};

pub enum TransportLayerConnection {
    RakNet(rak_rs::connection::Connection),
    // TOOD NetherNet(nethernet::connection::Connection),
    // TODO Quic(s2n_quic::connection::Connection),
    // TODO Tcp(net::TcpStream),
}

impl TransportLayerConnection {
    pub async fn send(&mut self, stream: &[u8]) -> Result<(), TransportLayerError> {
        match self {
            TransportLayerConnection::RakNet(conn) => {
                // 1 = RAKNET_GAMEPACKET_ID
                let mut str = Vec::with_capacity(stream.len() + 1);

                str.write_u8(RAKNET_GAMEPACKET_ID)?;
                str.write_all(stream)?;

                // TODO Find out if immediate: true should be used
                conn.send(str.as_slice(), true)
                    .await
                    .map_err(|err| TransportLayerError::RakNetError(RaknetError::SendError(err)))?;
            }
        }

        Ok(())
    }

    pub async fn recv(&mut self) -> Result<Vec<u8>, TransportLayerError> {
        let stream = match self {
            TransportLayerConnection::RakNet(conn) => {
                let stream = conn
                    .recv()
                    .await
                    .map_err(|e| TransportLayerError::RakNetError(RaknetError::RecvError(e)))?;

                let mut stream = Cursor::new(stream);

                // Read the RakNet packet id
                // TODO: Should we check if this is the correct one?
                stream.read_u8()?;
                
                let mut stream = stream.into_inner();
                stream.drain(..1);
                
                stream
            }
        };

        Ok(stream)
    }

    pub async fn close(self) {
        match self {
            TransportLayerConnection::RakNet(conn) => {
                conn.close().await;
            }
        }
    }
}
