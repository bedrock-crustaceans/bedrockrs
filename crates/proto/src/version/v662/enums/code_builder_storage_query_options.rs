pub mod CodeBuilderStorageQueryOptions {
    use bedrockrs_macros::ProtoCodec;
    
    #[derive(ProtoCodec)]
    #[enum_repr(i8)]
    #[repr(i8)]
    pub enum Category {
        None = 0,
        CodeStatus = 1,
        Instantiation = 2,
    }
    #[derive(ProtoCodec)]
    #[enum_repr(i8)]
    #[repr(i8)]
    pub enum Operation {
        None = 0,
        Get = 1,
        Set = 2,
        Reset = 3,
    }
}