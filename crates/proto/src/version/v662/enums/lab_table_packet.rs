pub mod LabTablePacket {
    use bedrockrs_macros::ProtoCodec;
    
    #[derive(ProtoCodec)]
    pub enum Type {
        StartCombine = 0,
        StartReaction = 1,
        Reset = 2,
    }
}