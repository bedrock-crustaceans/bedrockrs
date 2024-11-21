pub mod ItemReleaseInventoryTransaction {
    use bedrockrs_macros::ProtoCodec;
    
    #[derive(ProtoCodec)]
    pub enum ActionType {
        Release = 0,
        Use = 1,
    }
}