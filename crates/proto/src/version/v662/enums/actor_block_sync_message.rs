pub mod ActorBlockSyncMessage {
    use bedrockrs_macros::ProtoCodec;
    #[derive(ProtoCodec)]
    pub enum MessageId {
        NONE = 0,
        CREATE = 1,
        DESTROY = 2,
    }
}
