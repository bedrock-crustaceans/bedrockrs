use serialize::error::SerilizationError;
use serialize::proto::ser::MCProtoSerialize;

#[derive(Debug)]
pub struct PackURL {
    uuid: String,
    version: String,
    url: String,
}

impl MCProtoSerialize for PackURL {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        let uuid_version = format!("{}_{}", self.uuid, self.version);

        match uuid_version.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        match self.url.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        Ok(())
    }
}
