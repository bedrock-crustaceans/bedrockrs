pub mod ComplexInventoryTransaction {
    use bedrockrs_macros::ProtoCodec;

    #[derive(ProtoCodec)]
    pub enum Type {
        NormalTransaction = 0,
        InventoryMismatch = 1,
        ItemUseTransaction = 2,
        ItemUseOnEntityTransaction = 3,
        ItemReleaseTransaction = 4,
    }
}
