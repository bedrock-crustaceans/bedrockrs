use crate::helper::ProtoHelper;
use crate::version::v729::gamepackets::GamePackets;

pub struct ProtoHelperV662;

impl ProtoHelper for ProtoHelperV662 {
    type GamePacketType = GamePackets;
}
