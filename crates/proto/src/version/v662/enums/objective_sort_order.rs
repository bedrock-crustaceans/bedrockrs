use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum ObjectiveSortOrder {
    Ascending = 0,
    Descending = 1,
}