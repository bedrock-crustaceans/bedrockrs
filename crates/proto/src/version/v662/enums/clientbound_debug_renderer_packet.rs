pub mod ClientboundDebugRendererPacket {
    use bedrockrs_macros::ProtoCodec;
    
    #[derive(ProtoCodec)]
    pub enum Type {
        Invalid = 0,
        ClearDebugMarkers = 1,
        AddDebugMarkerCube = 2,
    }
}