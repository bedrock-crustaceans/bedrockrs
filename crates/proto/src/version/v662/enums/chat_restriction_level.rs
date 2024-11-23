use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum ChatRestrictionLevel {
    None = 0,
    Dropped = 1,
    Disabled = 2,
}
