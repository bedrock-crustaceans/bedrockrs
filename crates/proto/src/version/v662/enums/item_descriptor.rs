pub mod ItemDescriptor {
    use bedrockrs_macros::ProtoCodec;

    #[derive(ProtoCodec)]
    #[enum_repr(i8)]
    #[repr(i8)]
    pub enum InternalType {
        Invalid = 0,
        Default = 1,
        Molang = 2,
        ItemTag = 3,
        Deferred = 4,
        ComplexAlias = 5,
    }
}