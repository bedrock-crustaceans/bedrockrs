use std::collections::BTreeMap;

use bedrock_core::read::ByteStreamRead;
use bedrock_core::write::ByteStreamWrite;
use proto_core::error::ProtoCodecError;
use proto_core::ProtoCodec;
use serde_json::Value;

#[derive(Debug)]
pub struct HandshakeServerToClientPacket {
    pub handshake_jwt: BTreeMap<String, Value>,
}

impl ProtoCodec for HandshakeServerToClientPacket {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError> {
        todo!()
    }

    fn proto_deserialize(cursor: &mut ByteStreamRead) -> Result<Self, ProtoCodecError> {
        todo!()
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
