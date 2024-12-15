pub mod EmotePacket {
    use bedrockrs_macros::ProtoCodec;
    
    #[derive(ProtoCodec)]
    pub enum Flags {
        ServerSide = 1 << 0,
        MuteEmoteChat = 1 << 1,
    }
}