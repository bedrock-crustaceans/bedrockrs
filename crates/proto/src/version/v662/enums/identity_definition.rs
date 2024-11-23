pub mod IdentityDefinition {
    use bedrockrs_macros::ProtoCodec;
    
    #[derive(ProtoCodec)]
    pub enum Type {
        Invalid = 0,
        Player = 1,
        Entity = 2,
        FakePlayer = 3,
    }
}