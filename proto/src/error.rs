use proto_core::error::ProtoCodecError;
use rak_rs::connection::queue::SendQueueError;
use rak_rs::connection::RecvError;
use rak_rs::error::server::ServerError;

#[derive(Debug)]
pub enum ListenerError {
    AddrBindError,
    AlreadyOnline,
    NotListening,
    TransportListenerError(TransportLayerError),
}

#[derive(Debug)]
pub enum ConnectionError {
    IOError(std::io::Error),
    ProtoCodecError(ProtoCodecError),
    ConnectionClosed,
    TransportError(TransportLayerError),
    CompressError(CompressionError),
    InvalidRakNetHeader,
    UnknownCompressionMethod(u8),
    WrongCompressionMethod,
}

#[derive(Debug)]
pub enum CompressionError {
    ZlibError(Box<dyn std::error::Error>),
    SnappyError(std::io::Error),
    IOError(std::io::Error),
}

#[derive(Debug)]
pub enum LoginError {
    ConnError(ConnectionError),
    Abort { reason: String },
    WrongProtocolVersion { client: i32, server: Vec<i32> },
    FormatError(String),
}

#[derive(Debug)]
pub enum TransportLayerError {
    IOError(std::io::Error),
    RaknetUDPError(RaknetError),
}

#[derive(Debug)]
pub enum RaknetError {
    RecvError(RecvError),
    SendError(SendQueueError),
    ServerError(ServerError),
    FormatError(String),
}
