use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
#[enum_repr(i8)]
pub enum MapDecorationType {
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

impl MapDecorationType {
    const PLAYER: MapDecorationType = MapDecorationType::MarkerWhite;
    const PLAYER_OFF_MAP: MapDecorationType = MapDecorationType::SquareWhite;
    const PLAYER_OFF_LIMITS: MapDecorationType = MapDecorationType::SmallSquareWhite;
    const PLAYER_HIDDEN: MapDecorationType = MapDecorationType::NoDraw;
    const ITEM_FRAME: MapDecorationType = MapDecorationType::MarkerGreen;
}

#[derive(ProtoCodec)]
pub struct MapDecoration {
    pub map_decoration_type: MapDecorationType,
    pub rotation: i8,
    pub x: i8,
    pub y: i8,
    pub label: String,
    #[endianness(var)]
    pub color_argb: u32
}