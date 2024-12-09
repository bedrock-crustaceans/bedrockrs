pub mod InventorySource {
    use bedrockrs_macros::ProtoCodec;

    #[derive(ProtoCodec, Clone, Debug)]
    pub enum InventorySourceFlags {
        NoFlag = 0,
        WorldInteractionRandom = 1,
    }
}