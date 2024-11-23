use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum ShowStoreOfferRedirectType {
    MarketplaceOffer = 0,
    DressingRoomOffer = 1,
    ThirdPartyServerPage = 2,
    Count = 3,
}