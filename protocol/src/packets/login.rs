use bedrock_core::types::*;
use serialize::proto::de::MCProtoDeserialize;
use serialize::proto::ser::MCProtoSerialize;
use serialize_derive::{MCProtoDeserialize, MCProtoSerialize};

use crate::types::connection_request::ConnectionRequestType;

#[derive(Debug, MCProtoSerialize, MCProtoDeserialize)]
pub struct LoginPacket {
    pub client_network_version: i32be,
    pub connection_request: ConnectionRequestType,
}
