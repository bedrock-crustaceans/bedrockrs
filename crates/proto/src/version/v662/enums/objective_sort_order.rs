use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum ObjectiveSortOrder {
    Ascending = 0,
    Descending = 1,
}