pub mod RequestAbilityPacket {
    use bedrockrs_macros::ProtoCodec;
    
    #[derive(ProtoCodec)]
    pub enum Type {
        Unset = 0,
        Bool = 1,
        Float = 2,
    }
}