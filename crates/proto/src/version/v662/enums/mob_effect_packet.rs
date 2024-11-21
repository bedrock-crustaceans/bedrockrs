pub mod MobEventPacket {
    use bedrockrs_macros::ProtoCodec;
    
    #[derive(ProtoCodec)]
    pub enum Event {
        Invalid = 0,
        Add = 1,
        Update = 2,
        Remove = 3,
    }
}