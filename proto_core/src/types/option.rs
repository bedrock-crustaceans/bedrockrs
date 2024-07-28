use std::io::Cursor;

use crate::error::ProtoCodecError;
use crate::ProtoCodec;

impl<T: ProtoCodec> ProtoCodec for Option<T> {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match self {
            None =>  false.proto_serialize(buf)?,
            Some(v) => {
                true.proto_serialize(buf)?;
                v.proto_serialize(buf)?;
            }
        };

        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        Ok(match bool::proto_deserialize(stream)? {
            false => None,
            true => T::proto_deserialize(stream)?
        })
    }
}
