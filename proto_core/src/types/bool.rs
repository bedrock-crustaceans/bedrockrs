use bedrock_core::read::ByteStreamRead;
use bedrock_core::write::ByteStreamWrite;
use bedrock_core::LE;

use crate::error::ProtoCodecError;
use crate::ProtoCodec;

impl ProtoCodec for bool {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match self {
            true => match LE::<u8>::new(1).write(buf) {
                Ok(_) => Ok(()),
                Err(e) => Err(ProtoCodecError::IOError(e)),
            },
            false => match LE::<u8>::new(0).write(buf) {
                Ok(_) => Ok(()),
                Err(e) => Err(ProtoCodecError::IOError(e)),
            },
        }
    }

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        // a bool is represented as a byte
        return match LE::<u8>::read(stream) {
            Ok(v) => {
                match v.into_inner() {
                    // 0 is counted as false
                    0 => Ok(false),
                    // Anything above 0 is true
                    _ => Ok(true),
                }
            }
            Err(e) => Err(ProtoCodecError::IOError(e)),
        };
    }
}
