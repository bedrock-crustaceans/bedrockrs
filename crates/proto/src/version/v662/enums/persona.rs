pub mod persona {
    use bedrockrs_macros::ProtoCodec;
    
    #[derive(ProtoCodec)]
    pub enum AnimatedTextureType {
        None = 0,
        Face = 1,
        Body32x32 = 2,
        Body128x128 = 3,
    }
    
    #[derive(ProtoCodec)]
    pub enum AnimationExpression {
        Linear = 0,
        Blinking = 1,
    }
}