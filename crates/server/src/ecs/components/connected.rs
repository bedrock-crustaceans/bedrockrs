use shipyard::Component;
use bedrockrs_proto::connection::shard::arc::ConnectionShared;
use bedrockrs_proto::helper::ProtoHelper;

#[derive(Component)]
pub struct Connected<T: ProtoHelper + 'static> where <T as ProtoHelper>::GamePacketType: Sync {
    pub connection: ConnectionShared<T>,
}
