pub mod PositionTrackingDBServerBroadcastPacket {
    use bedrockrs_macros::ProtoCodec;
    
    #[derive(ProtoCodec)]
    pub enum Action {
        Update = 0,
        Destroy = 1,
        NotFound = 2,
    }
}