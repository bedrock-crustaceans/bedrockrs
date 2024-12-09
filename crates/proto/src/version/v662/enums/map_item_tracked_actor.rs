pub mod MapItemTrackedActor {
    use bedrockrs_macros::ProtoCodec;

    #[derive(ProtoCodec, Clone, Debug)]
    pub enum Type {
        Entity = 0,
        BlockEntity = 1,
        Other = 2,
        Count = 3,
    }
}