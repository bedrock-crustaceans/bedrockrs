use bedrockrs_proto_core::error::ProtoCodecError;

#[derive(Debug, Clone)]
pub enum SubClientID {
    PrimaryClient,
    Client2,
    Client3,
    Client4,
}

impl TryFrom<u8> for SubClientID {
    type Error = ProtoCodecError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SubClientID::PrimaryClient),
            1 => Ok(SubClientID::Client2),
            2 => Ok(SubClientID::Client3),
            3 => Ok(SubClientID::Client4),
            other => Err(ProtoCodecError::InvalidEnumID(
                format!("{other:?}"),
                String::from("SubClientID"),
            )),
        }
    }
}

impl From<SubClientID> for u8 {
    fn from(value: SubClientID) -> Self {
        match value {
            SubClientID::PrimaryClient => 0,
            SubClientID::Client2 => 1,
            SubClientID::Client3 => 2,
            SubClientID::Client4 => 3,
        }
    }
}
