use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum CommandParameterOption {
    None = 0,
    EnumAutocompleteExpansion = 0x01,
    HasSemanticConstraint = 0x02,
    EnumAsChainedCommand = 0x04,
}