use std::io::Cursor;

use crate::error::DeserilizationError;

///
pub trait MCProtoDeserialize {
    fn proto_deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized;
}
