pub mod AnimatePacket {
    use bedrockrs_macros::ProtoCodec;
    
    #[derive(ProtoCodec)]
    pub enum Action {
        NoAction = 0,
        Swing = 1,
        WakeUp = 3,
        CriticalHit = 4,
        MagicCriticalHit = 5,
        RowRight = 128,
        RowLeft = 129,
    }
}