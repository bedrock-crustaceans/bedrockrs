pub mod InteractPacket {
    use bedrockrs_macros::ProtoCodec;
    
    #[derive(ProtoCodec)]
    pub enum Action {
        Invalid = 0,
        StopRiding = 3,
        InteractUpdate = 4,
        NpcOpen = 5,
        OpenInventory = 6,
    }
}