use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum ItemStackRequestActionType {
    Take = 0,
    Place = 1,
    Swap = 2,
    Drop = 3,
    Destroy = 4,
    Consume = 5,
    Create = 6,
    PlaceInItemContainer = 7,
    TakeFromItemContainer = 8,
    ScreenLabTableCombine = 9,
    ScreenBeaconPayment = 10,
    ScreenHUDMineBlock = 11,
    CraftRecipe = 12,
    CraftRecipeAuto = 13,
    CraftCreative = 14,
    CraftRecipeOptional = 15,
    CraftRepairAndDisenchant = 16,
    CraftLoom = 17,
    #[deprecated = "Ask Tylaing"] CraftNonImplemented = 18,
    #[deprecated = "Ask Tylaing"] CraftResults = 19,
    Ifdef = 20,
    TestIntrastructureEnabled = 21,
    Test = 22,
    Endif = 23,
}