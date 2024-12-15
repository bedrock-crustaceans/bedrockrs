use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::ObjectiveSortOrder;

#[gamepacket(id = 107)]
#[derive(ProtoCodec)]
pub struct SetDisplayObjectivePacket {
    pub display_slot_name: String,
    pub objective_name: String,
    pub objective_display_name: String,
    pub criteria_name: String,
    pub sort_order: ObjectiveSortOrder,
}