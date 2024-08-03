use crate::types::network_block_pos::NetworkBlockPos;
use bedrockrs_core::int::VAR;
use bedrockrs_shared::actor_runtime_id::ActorRuntimeID;

#[derive(Debug, Clone)]
pub struct PlayerActionPacket {
    pub player_runtime_id: ActorRuntimeID,
    pub action: VAR<i32>,
    pub block_pos: NetworkBlockPos,
    pub result_pos: NetworkBlockPos,
    pub face: VAR<i32>,
}
use bedrockrs_proto_core::ProtoCodec;
impl ProtoCodec for PlayerActionPacket {
    fn proto_serialize(
        &self,
        stream: &mut Vec<u8>,
    ) -> Result<(), bedrockrs_proto_core::error::ProtoCodecError> {
        Ok(())
    }

    fn proto_deserialize(
        stream: &mut std::io::Cursor<&[u8]>,
    ) -> Result<Self, bedrockrs_proto_core::error::ProtoCodecError> {
        Ok(Self {
            player_runtime_id: ActorRuntimeID::proto_deserialize(stream)?,
            action: VAR::<i32>::proto_deserialize(stream)?,
            block_pos: NetworkBlockPos::proto_deserialize(stream)?,
            result_pos: NetworkBlockPos::proto_deserialize(stream)?,
            face: VAR::<i32>::proto_deserialize(stream)?,
        })
    }
}
