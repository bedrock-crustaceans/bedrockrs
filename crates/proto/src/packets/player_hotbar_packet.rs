use bedrockrs_core::int::VAR;
use bedrockrs_proto_core::ProtoCodec;

use crate::types::container_id::ContainerID;

#[derive(Debug, Clone)]
pub struct PlayerHotbarPacket {
    pub selected_slot: VAR<u32>,
    pub container_id: ContainerID,
    pub should_select_slot: bool,
}

impl ProtoCodec for PlayerHotbarPacket {
    fn proto_serialize(
        &self,
        stream: &mut Vec<u8>,
    ) -> Result<(), bedrockrs_proto_core::error::ProtoCodecError> {
        unimplemented!()
    }

    fn proto_deserialize(
        stream: &mut std::io::Cursor<&[u8]>,
    ) -> Result<Self, bedrockrs_proto_core::error::ProtoCodecError> {
        unimplemented!()
    }
}
