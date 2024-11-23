use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Eq, PartialEq)]
pub enum AttributeOperands {
    Min = 0,
    Max = 1,
    Current = 2,
    TotalOperands = 3,
}

impl AttributeOperands {
    pub const OPERAND_INVALID: AttributeOperands = AttributeOperands::TotalOperands;
}