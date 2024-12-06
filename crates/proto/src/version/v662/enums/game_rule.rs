pub mod GameRule {
    use bedrockrs_macros::ProtoCodec;
    
    #[derive(ProtoCodec)]
    pub enum Type {
        Invalid = 0,
        Bool = 1,
        Int = 2,
        Float = 3,
    }
}