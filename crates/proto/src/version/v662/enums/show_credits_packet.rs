pub mod ShowCreditsPacket {
    use bedrockrs_macros::ProtoCodec;
    
    #[derive(ProtoCodec)]
    pub enum CreditsState {
        Start = 0,
        Finished = 1,
    }
}