pub mod ClientboundMapItemDataPacket {
    use bedrockrs_macros::ProtoCodec;
    
    #[derive(ProtoCodec)]
    pub enum Type {
        Invalid = 0,
        TextureUpdate = 1 << 1,
        DecorationUpdate = 1 << 2,
        Creation = 1 << 3,
    }
}