use std::error::Error;

use proto_core::error::ProtoCodecError;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ListenerError {
    AddrBindError,
    AlreadyOnline,
    NotListening,
}

#[derive(Debug)]
pub enum ConnectionError {
    ReadIOError,
    WriteIOError,
    ProtoCodecError(ProtoCodecError),
    ConnectionClosed,
    RakNetError,
    CompressError(CompressionError),
    InvalidRakNetHeader,
    UnknownCompressionMethod(u8),
    WrongCompressionMethod,
}

#[derive(Debug)]
pub enum CompressionError {
    ZlibError(Box<dyn Error>),
    SnappyError(snap::Error),
    InvalidCompressionMethod,
}

#[derive(Debug)]
pub enum LoginError {
    ConnError(ConnectionError),
    WrongProtocolVersion { client: i32 },
    PacketMismatch(String),
}
