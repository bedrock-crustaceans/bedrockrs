pub mod IdentityDefinition {
    use bedrockrs_macros::ProtoCodec;
    use crate::version::v662::types::ActorUniqueID;
    
    #[derive(ProtoCodec)]
    #[enum_repr(i8)]
    #[repr(i8)]
    pub enum Type {
        Invalid = 0,
        Player {
            #[endianness(var)]
            player_unique_id: i64,
        } = 1,
        Entity {
            actor_id: ActorUniqueID,
        } = 2,
        FakePlayer {
            fake_player_name: String,
        } = 3,
    }
}