use bedrockrs_core::read::ByteStreamRead;
use bedrockrs_core::write::ByteStreamWrite;

use crate::error::ProtoCodecError;
use crate::ProtoCodec;

impl<T: ProtoCodec> ProtoCodec for Option<T> {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match self {
            None => match false.proto_serialize(buf) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
            Some(v) => {
                match true.proto_serialize(buf) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                };

                match v.proto_serialize(buf) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
            }
        }
    }

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        match bool::proto_deserialize(stream) {
            Ok(v) => match v {
                false => Ok(None),
                true => match T::proto_deserialize(stream) {
                    Ok(v) => Ok(Some(v)),
                    Err(e) => Err(e),
                },
            },
            Err(e) => Err(e),
        }
    }
}
