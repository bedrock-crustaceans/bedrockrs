use std::convert::TryInto;
use std::io::{Cursor, Read, Write};
use std::sync::Arc;


use bedrockrs_core::VAR;

use crate::error::ProtoCodecError;
use crate::ProtoCodec;

impl ProtoCodec for String {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        let len = match self.len().try_into() {
            Ok(v) => v,
            Err(e) => {
                return Err(ProtoCodecError::FromIntError(e));
            }
        };

        match VAR::<u32>::new(len).write(buf) {
            Ok(_) => {}
            Err(e) => return Err(ProtoCodecError::IOError(Arc::new(e))),
        };

        match buf.write_all(self.as_bytes()) {
            Ok(_) => {}
            Err(e) => return Err(ProtoCodecError::IOError(Arc::new(e))),
        };

        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        let len = match VAR::<u32>::read(stream) {
            Ok(v) => v.into_inner(),
            Err(e) => {
                return Err(ProtoCodecError::IOError(Arc::new(e)));
            }
        };

        let len = match len.try_into() {
            Ok(v) => v,
            Err(e) => {
                return Err(ProtoCodecError::FromIntError(e));
            }
        };

        let mut string_buf = vec![0u8; len];

        match stream.read_exact(&mut *string_buf) {
            Ok(_) => {}
            Err(e) => {
                return Err(ProtoCodecError::IOError(Arc::new(e)));
            }
        }

        match String::from_utf8(string_buf) {
            Ok(str) => Ok(str),
            Err(e) => Err(ProtoCodecError::UTF8Error(e)),
        }
    }
}
