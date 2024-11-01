use std::io::{Cursor, Read};
use byteorder::{LittleEndian, ReadBytesExt};
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;

#[derive(Debug, Clone)]
pub struct ConnectionRequest {
    pub certificate_chain: Vec<String>,
    pub raw_token: String,
}

fn read_i32_string(stream: &mut Cursor<&[u8]>) -> Result<String, ProtoCodecError> {
    let len = stream
        .read_i32::<LittleEndian>()?
        .try_into()
        .map_err(ProtoCodecError::FromIntError)?;

    let mut string_buf = vec![0; len];
    stream.read_exact(&mut string_buf)?;

    Ok(String::from_utf8(string_buf)?)
}

impl ProtoCodec for ConnectionRequest {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        todo!()
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        stream.read_u32::<LittleEndian>()?;

        let certificate_chain = read_i32_string(stream)?;
        let certificate_chain = serde_json::from_str(&certificate_chain)?;
        
        let raw_token = read_i32_string(stream)?;
        
        Ok(Self {
            certificate_chain,
            raw_token,
        })
    }

    fn get_size_prediction(&self) -> usize {
        let mut size = 0;

        for element in &self.certificate_chain {
            size += element.get_size_prediction();
        }
        
        size += self.raw_token.get_size_prediction();
        
        size
    }
}
