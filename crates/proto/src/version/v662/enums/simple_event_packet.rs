pub mod SimpleEventPacket {
    use bedrockrs_macros::ProtoCodec;
    
    #[derive(ProtoCodec)]
    pub enum Subtype {
        UninitializedSubtype = 0,
        EnableCommands = 1,
        DisableCommands = 2,
        UnlockWorldTemplateSettings = 3,
    }
}