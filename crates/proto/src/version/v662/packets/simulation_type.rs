use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::SimulationType;

#[gamepacket(id = 168)]
#[derive(ProtoCodec)]
pub struct SimulationTypePacket {
    pub sim_type: SimulationType,
}