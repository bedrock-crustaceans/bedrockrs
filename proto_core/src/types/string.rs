use std::convert::TryInto;
use std::io::{Read, Write};
use std::sync::Arc;

use bedrock_core::read::ByteStreamRead;
use bedrock_core::VAR;
use bedrock_core::write::ByteStreamWrite;

use crate::error::ProtoCodecError;
use crate::ProtoCodec;

impl ProtoCodec for String {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
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

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
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
