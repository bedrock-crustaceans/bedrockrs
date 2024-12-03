pub mod ComplexInventoryTransaction {
    use bedrockrs_macros::ProtoCodec;

    #[derive(ProtoCodec)]
    #[enum_repr(u32)]
    #[enum_endianness(var)]
    #[repr(u32)]
    pub enum Type {
        NormalTransaction = 0,
        InventoryMismatch = 1,
        ItemUseTransaction = 2,
        ItemUseOnEntityTransaction = 3,
        ItemReleaseTransaction = 4,
    }
}
