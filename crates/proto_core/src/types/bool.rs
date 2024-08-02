use std::io::Cursor;
use std::sync::Arc;

use bedrockrs_core::int::LE;

use crate::error::ProtoCodecError;
use crate::ProtoCodec;

impl ProtoCodec for bool {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match self {
            true => LE::<u8>::new(1)
                .write(buf)
                .map_err(|e| ProtoCodecError::IOError(Arc::new(e))),
            false => LE::<u8>::new(0)
                .write(buf)
                .map_err(|e| ProtoCodecError::IOError(Arc::new(e))),
        }
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        // a Bool is represented as a byte
        Ok(
            match LE::<u8>::read(stream)
                .map_err(|e| ProtoCodecError::IOError(Arc::new(e)))?
                .into_inner()
            {
                // 0 is counted as false
                0 => false,
                // Anything above 0 is true
                _ => true,
            },
        )
    }
}
