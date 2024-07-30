use std::convert::TryInto;
use std::io::{Cursor, Read, Write};
use std::sync::Arc;

use bedrockrs_core::int::VAR;

use crate::error::ProtoCodecError;
use crate::ProtoCodec;

impl ProtoCodec for String {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        let len = self
            .len()
            .try_into()
            .map_err(|e| ProtoCodecError::FromIntError(e))?;

        VAR::<u32>::new(len)
            .write(buf)
            .map_err(|e| ProtoCodecError::IOError(Arc::new(e)))?;
        buf.write_all(self.as_bytes())
            .map_err(|e| ProtoCodecError::IOError(Arc::new(e)))?;

        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        let len = VAR::<u32>::read(stream)
            .map_err(|e| ProtoCodecError::IOError(Arc::new(e)))?
            .into_inner();
        let len = len
            .try_into()
            .map_err(|e| ProtoCodecError::FromIntError(e))?;

        let mut string_buf = vec![0u8; len];

        stream
            .read_exact(&mut *string_buf)
            .map_err(|e| ProtoCodecError::IOError(Arc::new(e)))?;
        String::from_utf8(string_buf).map_err(|e| ProtoCodecError::UTF8Error(e))
    }
}
