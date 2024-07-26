use std::io::{Cursor, Write};
use std::sync::Arc;

use bedrockrs_core::LE;

use crate::error::{RaknetError, TransportLayerError};
use crate::info::RAKNET_GAME_PACKET_ID;

///
pub enum TransportLayerConnection {
    RaknetUDP(rak_rs::connection::Connection),
    // TODO RaknetTCP(...),
    NetherNet(/* TODO */),
    // TODO Quic(s2n_quic::connection::Connection),
    // TODO Tcp(net::TcpStream),
    // TODO Udp(net::UdpSocket)
}

impl TransportLayerConnection {
    pub async fn send(&mut self, stream: &Cursor<&[u8]>) -> Result<(), TransportLayerError> {
        match self {
            TransportLayerConnection::RaknetUDP(conn) => {
                let mut final_stream = vec![];

                LE::<u8>::write(&LE::new(RAKNET_GAME_PACKET_ID), &mut final_stream)
                    .map_err(|e| TransportLayerError::IOError(Arc::new(e)))?;

                final_stream
                    .write_all(stream.get_ref())
                    .map_err(|e| TransportLayerError::IOError(Arc::new(e)))?;

                // TODO Find out if immediate: true should be used
                conn.send(final_stream.as_slice(), true)
                    .await
                    .map_err(|e| TransportLayerError::RaknetUDPError(RaknetError::SendError(e)))
            }
            _ => {
                todo!()
            }
        }
    }

    pub async fn recv(&mut self, stream: &mut Vec<u8>) -> Result<(), TransportLayerError> {
        match self {
            TransportLayerConnection::RaknetUDP(conn) => {
                let mut recv_stream = conn
                    .recv()
                    .await
                    .map_err(|e| TransportLayerError::RaknetUDPError(RaknetError::RecvError(e)))?;

                let mut recv_stream = Cursor::new(recv_stream.as_slice());

                match LE::<u8>::read(&mut recv_stream)
                    .map_err(|e| TransportLayerError::IOError(Arc::new(e)))?
                    .into_inner()
                {
                    RAKNET_GAME_PACKET_ID => {}
                    other => {
                        return Err(TransportLayerError::RaknetUDPError(
                            RaknetError::FormatError(format!(
                                "Expected Raknet Game Packet ID ({:?}), got: {:?}",
                                RAKNET_GAME_PACKET_ID, other
                            )),
                        ));
                    }
                };

                Ok(stream
                    .write_all(recv_stream.into_inner())
                    .map_err(|e| TransportLayerError::IOError(Arc::new(e)))?)
            }

            _ => {
                todo!()
            }
        }
    }

    pub async fn close(self) {
        match self {
            TransportLayerConnection::RaknetUDP(conn) => {
                conn.close().await;
            }
            _ => {
                todo!()
            }
        }
    }
}
