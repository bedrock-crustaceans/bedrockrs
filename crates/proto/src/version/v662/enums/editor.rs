pub mod Editor {
    use bedrockrs_macros::ProtoCodec;
    
    #[derive(ProtoCodec)]
    pub enum WorldType {
        NonEditor = 0,
        EditorProject = 1,
        EditorTestLevel = 2,
    }
}