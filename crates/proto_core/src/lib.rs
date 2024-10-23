extern crate core;

use std::io::Cursor;

use crate::error::ProtoCodecError;

mod endian;
use crate::sub_client::SubClientID;
pub use endian::*;

pub mod error;
pub mod sub_client;
pub mod types;

pub trait ProtoCodec: Sized {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError>;

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError>;

    fn get_size_prediction(&self) -> usize;
}

pub trait GamePacket: Sized + ProtoCodec {
    const ID: u16;
    const COMPRESS: bool;
    const ENCRYPT: bool;

    #[inline]
    fn get_size_prediction(&self) -> usize {
        <Self as ProtoCodec>::get_size_prediction(self)
    }
}

pub trait GamePacketsAll: Sized {
    fn compress(&self) -> bool;
    fn encrypt(&self) -> bool;

    fn pk_serialize(
        &self,
        stream: &mut Vec<u8>,
        subclient_sender_id: SubClientID,
        subclient_target_id: SubClientID,
    ) -> Result<(), ProtoCodecError>;
    fn pk_deserialize(
        stream: &mut Cursor<&[u8]>,
    ) -> Result<(Self, SubClientID, SubClientID), ProtoCodecError>;

    fn get_size_prediction(&self) -> usize;
}
