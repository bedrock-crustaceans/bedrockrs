use std::collections::BTreeMap;
use std::io::Cursor;

use serde_json::Value;

use serialize::error::{DeserilizationError, SerilizationError};
use serialize::proto::de::MCProtoDeserialize;
use serialize::proto::ser::MCProtoSerialize;

#[derive(Debug)]
pub struct HandshakeServerToClientPacket {
    pub handshake_webtoken: BTreeMap<String, Value>,
}

impl MCProtoSerialize for HandshakeServerToClientPacket {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        Ok(())
    }
}

impl MCProtoDeserialize for HandshakeServerToClientPacket {
    fn proto_deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        Err(DeserilizationError::ReadIOError)
    }
}

//
// impl MCProtoSerialize for HandshakeServerToClientPacket {
//     fn proto_serialize(&self) -> Result<Vec<u8>, SerilizationError>
//     where
//         Self: Sized,
//     {
//         let key = match EcdsaPrivateKey::generate(EcdsaAlgorithm::ES384) {
//             Ok(v) => v,
//             Err(_) => {
//                 return Err(SerilizationError::GenerateKeyError);
//             }
//         };
//
//         let mut jwt = jwtk::HeaderAndClaims::new_dynamic();
//
//         jwt.header_mut();
//
//         for (string, val) in self.handshake_webtoken.iter() {
//             jwt.insert(string, val.clone());
//         }
//
//         let token = match sign(&mut jwt, &key) {
//             Ok(v) => v,
//             Err(_) => {
//                 return Err(SerilizationError::JwtError);
//             }
//         };
//
//         println!("token:\n{}", token);
//
//         let mut buf = vec![];
//
//         match buf.write_u64_varint(token.len() as u64) {
//             Ok(_) => {}
//             Err(_) => {
//                 return Err(SerilizationError::WriteVarintError);
//             }
//         }
//
//         buf.put_slice(token.as_bytes());
//
//         Ok(buf)
//     }
// }
//
// impl MCProtoDeserialize for HandshakeServerToClientPacket {
//     fn proto_deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
//     where
//         Self: Sized,
//     {
//         todo!()
//     }
// }
