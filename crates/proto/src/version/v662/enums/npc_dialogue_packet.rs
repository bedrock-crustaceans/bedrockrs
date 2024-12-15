pub mod NpcDialoguePacket {
    use bedrockrs_macros::ProtoCodec;
    
    #[derive(ProtoCodec)]
    pub enum NpcDialogueActionType {
        Open = 0,
        Close = 1,
    }
}