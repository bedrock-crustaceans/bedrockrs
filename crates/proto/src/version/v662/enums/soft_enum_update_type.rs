use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum SoftEnumUpdateType {
    Add = 0,
    Remove = 1,
    Replace = 2,
}