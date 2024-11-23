pub mod ItemUseInventoryTransaction {
    use bedrockrs_macros::ProtoCodec;
    
    #[derive(ProtoCodec)]
    pub enum ActionType {
        Place = 0,
        Use = 1,
        Destroy = 2,
    }
}