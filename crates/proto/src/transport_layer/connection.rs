use crate::error::{RaknetError, TransportLayerError};
use crate::info::RAKNET_GAME_PACKET_ID;
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
                let mut final_stream = vec![];

                final_stream.write_u8(RAKNET_GAME_PACKET_ID)?;

                final_stream.write_all(stream)?;

                // TODO Find out if immediate: true should be used
                conn.send(final_stream.as_slice(), true)
                    .await
                    .map_err(|err| TransportLayerError::RakNetError(RaknetError::SendError(err)))?;
            }
        }

        Ok(())
    }

    pub async fn recv(&mut self, stream: &mut Vec<u8>) -> Result<(), TransportLayerError> {
        match self {
            TransportLayerConnection::RakNet(conn) => {
                let recv_stream = conn
                    .recv()
                    .await
                    .map_err(|e| TransportLayerError::RakNetError(RaknetError::RecvError(e)))?;

                let mut recv_stream = Cursor::new(recv_stream.as_slice());

                // Read the RakNet packet id
                // TODO: Should we check if this is the correct one?
                recv_stream.read_u8()?;

                stream.write_all(recv_stream.into_inner())?;
            }
        }

        Ok(())
    }

    pub async fn close(self) {
        match self {
            TransportLayerConnection::RakNet(conn) => {
                conn.close().await;
            }
        }
    }
}
