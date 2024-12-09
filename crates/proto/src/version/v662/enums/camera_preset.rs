pub mod CameraPreset {
    use bedrockrs_macros::ProtoCodec;

    #[derive(ProtoCodec, Clone, Debug)]
    pub enum AudioListener {
        Camera = 0,
        Player = 1,
    }
}