use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum CommandBlockMode {
    Normal = 0,
    Chain = 2,
    Repeating = 1,
}