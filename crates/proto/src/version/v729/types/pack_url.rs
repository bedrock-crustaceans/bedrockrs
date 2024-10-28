use std::io::Cursor;

use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;

#[derive(Debug, Clone)]
pub struct PackURL {
    pub uuid: String,
    pub version: String,
    pub url: String,
}

impl ProtoCodec for PackURL {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        let uuid_version = format!("{}_{}", self.uuid, self.version);

        uuid_version.proto_serialize(stream)?;
        self.url.proto_serialize(stream)?;

        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        let (uuid, version) = match String::proto_deserialize(stream)?.split_once("_") {
            None => {
                return Err(ProtoCodecError::FormatMismatch(
                    "Expected uuid and version of pack url to be seperated by an underscore",
                ));
            }
            Some((u, v)) => (u.to_string(), v.to_string()),
        };

        let url = String::proto_deserialize(stream)?;

        Ok(Self { uuid, version, url })
    }

    fn get_size_prediction(&self) -> usize {
        self.uuid.get_size_prediction()
            + self.version.get_size_prediction()
            + self.url.get_size_prediction()
    }
}
