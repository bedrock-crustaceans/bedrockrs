use std::io::Write;
use std::sync::Arc;

use bedrockrs_core::stream::read::ByteStreamRead;
use bedrockrs_core::stream::write::ByteStreamWrite;
use bedrockrs_core::LE;
use tokio::io::AsyncReadExt;

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
    pub async fn send(&mut self, stream: &ByteStreamRead<'_>) -> Result<(), TransportLayerError> {
        match self {
            TransportLayerConnection::RaknetUDP(conn) => {
                let mut final_stream = ByteStreamWrite::new();

                match LE::<u8>::write(&LE::new(RAKNET_GAME_PACKET_ID), &mut final_stream) {
                    Ok(_) => {}
                    Err(e) => return Err(TransportLayerError::IOError(Arc::new(e))),
                };

                match final_stream.write_all(stream.get_ref().as_slice()) {
                    Ok(_) => {}
                    Err(e) => return Err(TransportLayerError::IOError(Arc::new(e))),
                };

                // TODO Find out if immediate: true should be used
                match conn.send(final_stream.as_slice(), true).await {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        return Err(TransportLayerError::RaknetUDPError(RaknetError::SendError(
                            e,
                        )));
                    }
                }
            }
            _ => {
                todo!()
            }
        }
    }

    pub async fn recv(&mut self, stream: &mut ByteStreamWrite) -> Result<(), TransportLayerError> {
        match self {
            TransportLayerConnection::RaknetUDP(conn) => {
                let mut recv_stream = match conn.recv().await {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(TransportLayerError::RaknetUDPError(RaknetError::RecvError(
                            e,
                        )));
                    }
                };

                let mut recv_stream = ByteStreamRead::new(&recv_stream);

                match LE::<u8>::read(&mut recv_stream) {
                    Ok(v) => match v.into_inner() {
                        RAKNET_GAME_PACKET_ID => {}
                        other => {
                            return Err(TransportLayerError::RaknetUDPError(
                                RaknetError::FormatError(format!(
                                    "Expected Raknet Game Packet ID ({:?}), got: {:?}",
                                    RAKNET_GAME_PACKET_ID, other
                                )),
                            ));
                        }
                    },
                    Err(e) => {
                        return Err(TransportLayerError::IOError(Arc::new(e)));
                    }
                };

                match stream.write_all(recv_stream.into_inner()) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(TransportLayerError::IOError(Arc::new(e))),
                }
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
