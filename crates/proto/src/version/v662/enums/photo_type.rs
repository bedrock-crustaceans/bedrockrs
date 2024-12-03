use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum PhotoType {
    Portfolio = 0,
    PhotoItem = 1,
    Book = 2,
}