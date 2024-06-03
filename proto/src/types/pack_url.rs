use std::io::Cursor;
use bedrock_core::stream::read::ByteStreamRead;
use bedrock_core::stream::write::ByteStreamWrite;

use proto_core::error::ProtoCodecError;
use proto_core::ProtoCodec;

#[derive(Debug)]
pub struct PackURL {
    uuid: String,
    version: String,
    url: String,
}

impl ProtoCodec for PackURL {
    fn proto_serialize(&self, stream: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        let uuid_version = format!("{}_{}", self.uuid, self.version);

        match uuid_version.proto_serialize(stream) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        match self.url.proto_serialize(stream) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        Ok(())
    }

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError> {
        let (uuid, version) = match String::proto_deserialize(stream) {
            Ok(v) => match v.split_once("_") {
                None => {
                    return Err(ProtoCodecError::FormatMismatch(String::from(
                        "Expected uuid and version of pack url to be seperated by _",
                    )));
                }
                Some((u, v)) => (u.to_string(), v.to_string()),
            },
            Err(e) => {
                return Err(e);
            }
        };

        let url = match String::proto_deserialize(stream) {
            Ok(v) => v,
            Err(e) => {
                return Err(e);
            }
        };

        Ok(Self { uuid, version, url })
    }
}
