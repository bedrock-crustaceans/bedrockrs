pub mod ItemUseInventoryTransaction {
    use bedrockrs_macros::ProtoCodec;

    #[derive(ProtoCodec, Clone, Debug)]
    #[enum_repr(u32)]
    #[enum_endianness(var)]
    #[repr(u32)]
    pub enum ActionType {
        Place = 0,
        Use = 1,
        Destroy = 2,
    }
}