use std::io::Cursor;

use serialize::error::{DeserilizationError, SerilizationError};
use serialize::proto::de::MCProtoDeserialize;
use serialize::proto::ser::MCProtoSerialize;

/// Is a SemVer
#[derive(Debug)]
pub struct BaseGameVersion(pub String);

impl MCProtoSerialize for BaseGameVersion {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError> where Self: Sized {
        self.0.proto_serialize(buf)
    }
}

impl MCProtoDeserialize for BaseGameVersion {
    fn proto_deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError> where Self: Sized {
        Ok(BaseGameVersion(match String::proto_deserialize(cursor) {
            Ok(v) => { v }
            Err(e) => { return Err(e); }
        }))
    }
}