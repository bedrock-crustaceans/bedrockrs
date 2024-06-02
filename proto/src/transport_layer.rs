use std::future::Future;
use std::io::{Error, Write};

use bedrock_core::stream::read::ByteStreamRead;
use bedrock_core::stream::write::ByteStreamWrite;
use rak_rs::connection::RecvError;
use tokio::io::AsyncWriteExt;
use tokio::net;

use crate::error::{ConnTransportLayerError, RaknetError};
use crate::info::RAKNET_GAME_PACKET_ID;

///
pub enum ConnTransportLayer {
    RaknetUDP(rak_rs::connection::Connection),
    //TODO RaknetTCP(...),
    Quic(s2n_quic::connection::Connection),
    Tcp(net::TcpStream),
    Udp(net::UdpSocket)
}

impl ConnTransportLayer {
    pub async fn send(&mut self, stream: &ByteStreamRead) -> Result<(), ConnTransportLayerError> {
        match self {
            ConnTransportLayer::RaknetUDP(conn) => {
                let mut final_stream = ByteStreamWrite::new();

                match final_stream.write_u8(RAKNET_GAME_PACKET_ID) {
                    Ok(_) => {}
                    Err(e) => return Err(ConnTransportLayerError::IOError(e)),
                };

                match final_stream.write(stream.as_slice()) {
                    Ok(_) => {}
                    Err(e) => return Err(ConnTransportLayerError::IOError(e)),
                };

                // TODO Find out if immediate: true should be used
                match conn.send(final_stream.as_slice(), true) {
                    Ok(_) => {}
                    Err(e) => { return Err(ConnTransportLayerError::RaknetUDPError(RaknetError::SendError(e))) }
                };

                Ok(())
            }
            _ => { todo!() }
        }
    }

    pub async fn recv(&mut self) -> Result<ByteStreamRead, ConnTransportLayerError> {
        match self {
            ConnTransportLayer::RaknetUDP(conn) => {
                let mut stream = match conn.recv().await {
                    Ok(v) => { ByteStreamRead::from(v) }
                    Err(e) => { return Err(ConnTransportLayerError::RaknetUDPError(RaknetError::RecvError(e))) }
                };

                match stream.read_u8() {
                    Ok(RAKNET_GAME_PACKET_ID) => {}
                    Ok(other) => { return Err(ConnTransportLayerError::RaknetUDPError(RaknetError::FormatError(format!("Expected Raknet Game Packet ID ({:?}), got: {:?}", RAKNET_GAME_PACKET_ID, other)))) }
                    Err(e) => { return Err(ConnTransportLayerError::IOError(e)) }
                };

                Ok(stream)
            }
            _ => { todo!() }
        }
    }
}
