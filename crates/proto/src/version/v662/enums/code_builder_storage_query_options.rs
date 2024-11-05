pub mod CodeBuilderStorageQueryOptions {
    use bedrockrs_macros::ProtoCodec;
    
    #[derive(ProtoCodec)]
    pub enum Category {
        None = 0,
        CodeStatus = 1,
        Instantiation = 2,
    }
    #[derive(ProtoCodec)]
    pub enum Operation {
        None = 0,
        Get = 1,
        Set = 2,
        Reset = 3,
    }
}