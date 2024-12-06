use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 89)]
#[derive(ProtoCodec)]
pub struct AddBehaviourTreePacket {
    pub json_behaviour_tree_structure: String,
}