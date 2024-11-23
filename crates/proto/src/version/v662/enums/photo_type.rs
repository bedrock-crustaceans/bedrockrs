use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum PhotoType {
    Portfolio = 0,
    PhotoItem = 1,
    Book = 2,
}