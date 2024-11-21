pub mod NpcRequestPacket {
    use bedrockrs_macros::ProtoCodec;
    
    #[derive(ProtoCodec)]
    pub enum RequestType {
        SetActions = 0,
        ExecuteAction = 1,
        ExecuteClosingCommands = 2,
        SetName = 3,
        SetSkin = 4,
        SetInteractText = 5,
        ExecuteOpeningCommands = 6,
    }
}