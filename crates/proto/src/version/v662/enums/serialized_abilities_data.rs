pub mod SerializedAbilitiesData {
    use bedrockrs_macros::ProtoCodec;

    #[derive(ProtoCodec)]
    pub enum SerializedAbilitiesLayer {
        CustomCache = 0,
        Base = 1,
        Spectator = 2,
        Commands = 3,
        Editor = 4,
    }
}