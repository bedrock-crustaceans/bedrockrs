pub mod SubChunkPacket {
    use bedrockrs_macros::ProtoCodec;
    
    #[derive(ProtoCodec)]
    pub enum HeightMapDataType {
        NoData = 0,
        HasData = 1,
        AllTooHigh = 2,
        AllTooLow = 3,
    }
    
    #[derive(ProtoCodec)]
    pub enum SubChunkRequestResult {
        Undefined = 0,
        Success = 1,
        LevelChunkDoesntExist = 2,
        WrongDimension = 3,
        PlayerDoesntExist = 4,
        IndexOutOfBounds = 5,
        SuccessAllAir = 6,
    }
}