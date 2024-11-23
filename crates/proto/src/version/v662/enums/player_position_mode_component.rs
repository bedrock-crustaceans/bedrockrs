pub mod PlayerPositionModeComponent {
    use bedrockrs_macros::ProtoCodec;
    
    #[derive(ProtoCodec)]
    pub enum PositionMode {
        Normal = 0,
        Respawn = 1,
        Teleport = 2,
        OnlyHeadRot = 3,
    }
}