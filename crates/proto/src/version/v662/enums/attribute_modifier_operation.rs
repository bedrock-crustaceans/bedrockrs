use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Eq, PartialEq)]
pub enum AttributeModifierOperation {
    Addition = 0,
    MultiplyBase = 1,
    MultiplyTotal = 2,
    Cap = 3,
    TotalOperations = 4,
}

impl AttributeModifierOperation {
    pub const OPERATION_INVALID: AttributeModifierOperation = AttributeModifierOperation::TotalOperations;
}

