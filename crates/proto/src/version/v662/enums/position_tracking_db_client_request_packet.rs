pub mod PositionTrackingDBClientRequestPacket {
    use bedrockrs_macros::ProtoCodec;
    
    #[derive(ProtoCodec)]
    pub enum Action {
        Query = 0
    }
}