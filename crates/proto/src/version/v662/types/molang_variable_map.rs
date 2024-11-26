use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub struct MolangVariableMap {
    pub serialized_variable_map: String,
}