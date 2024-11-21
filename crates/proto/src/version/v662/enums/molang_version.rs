use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum MolangVersion {
    Invalid = -1,
    BeforeVersioning = 0,
    Initial = 1,
    FixedItemRemainingUseDurationQuery = 2,
    ExpressionErrorMessages = 3,
    UnexpectedOperatorErrors = 4,
    ConditionalOperatorAssociativity = 5,
    ComparisonAndLogicalOperatorPrecedence = 6,
    DivideByNegativeValue = 7,
    FixedCapeFlapAmountQuery = 8,
    QueryBlockPropertyRenamedToState = 9,
    DeprecateOldBlockQueryNames = 10,
    DeprecatedSnifferAndCamelQueries = 11,
    LeafSupportingInFirstSolidBlockBelow = 12,
    NumValidVersions = 13,
}

impl MolangVersion {
    const LATEST: MolangVersion = MolangVersion::NumValidVersions - 1;
    const HARDCODED_MOLANG: MolangVersion = MolangVersion::LATEST;
}