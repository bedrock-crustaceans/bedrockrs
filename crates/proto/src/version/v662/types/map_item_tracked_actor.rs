pub mod MapItemTrackedActor {
    use crate::version::v662::types::{ActorUniqueID, NetworkBlockPosition};
    use bedrockrs_macros::ProtoCodec;

    #[derive(ProtoCodec, Clone, Debug)]
    #[enum_repr(i32)]
    #[enum_endianness(le)]
    #[repr(i32)]
    pub enum Type {
        Entity(ActorUniqueID) = 0,
        BlockEntity(NetworkBlockPosition) = 1,
        Other = 2,
    }
    
    #[derive(ProtoCodec, Clone, Debug)]
    pub struct UniqueId {
        pub unique_id_type: Type,
    }
}