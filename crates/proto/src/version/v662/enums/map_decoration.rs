pub mod MapDecoration {
    use bedrockrs_macros::ProtoCodec;
    
    #[derive(ProtoCodec)]
    pub enum Type {
        MarkerWhite = 0,
        MarkerGreen = 1,
        MarkerRed = 2,
        MarkerBlue = 3,
        XWhite = 4,
        TriangleRed = 5,
        SquareWhite = 6,
        MarkerSign = 7,
        MarkerPink = 8,
        MarkerOrange = 9,
        MarkerYellow = 10,
        MarkerTeal = 11,
        TriangleGreen = 12,
        SmallSquareWhite = 13,
        Mansion = 14,
        Monument = 15,
        NoDraw = 16,
        VillageDesert = 17,
        VillagePlains = 18,
        VillageSavanna = 19,
        VillageSnowy = 20,
        VillageTaiga = 21,
        JungleTemple = 22,
        WitchHut = 23,
        Count = 24,
    }

    impl Type {
        const PLAYER: Type = Type::MarkerWhite;
        const PLAYER_OFF_MAP: Type = Type::SquareWhite;
        const PLAYER_OFF_LIMITS: Type = Type::SmallSquareWhite;
        const PLAYER_HIDDEN: Type = Type::NoDraw;
        const ITEM_FRAME: Type = Type::MarkerGreen;
    }
}