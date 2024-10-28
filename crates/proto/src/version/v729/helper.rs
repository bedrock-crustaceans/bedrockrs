use crate::helper::ProtoHelper;
use crate::version::v729::gamepackets::GamePackets;

pub struct ProtoHelperV729;

impl ProtoHelper for ProtoHelperV729 {
    type GamePacketType = GamePackets;
}
