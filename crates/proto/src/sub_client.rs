use bedrockrs_proto_derive::ProtoCodec;
use bedrockrs_core::int::LE;
use bedrockrs_proto_core::error::ProtoCodecError;

#[derive(Debug, Clone)]
pub enum SubClientID {
    PrimaryClient,
    Client2,
    Client3,
    Client4,
}

impl SubClientID {
    pub fn proto_from(val: u8) -> Result<Self, ProtoCodecError> {
        match val {
            0 => Ok(SubClientID::PrimaryClient),
            1 => Ok(SubClientID::Client2),
            2 => Ok(SubClientID::Client3),
            3 => Ok(SubClientID::Client4),
            other => Err(ProtoCodecError::InvalidEnumID(
                format!("{other:?}"),
                String::from("SubClientID"),
            ))
        }
    }
    
    pub fn proto_to(&self) -> u8 {
        match self {
            SubClientID::PrimaryClient => 0, 
            SubClientID::Client2 => 1, 
            SubClientID::Client3 => 2, 
            SubClientID::Client4 => 3, 
        }
    }
}
