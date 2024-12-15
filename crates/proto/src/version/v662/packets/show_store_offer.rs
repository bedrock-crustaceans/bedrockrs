use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::ShowStoreOfferRedirectType;

#[gamepacket(id = 91)]
#[derive(ProtoCodec)]
pub struct ShowStoreOfferPacket {
    pub product_id: String,
    pub redirect_type: ShowStoreOfferRedirectType,
}