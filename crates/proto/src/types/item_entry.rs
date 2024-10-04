use bedrockrs_macros::ProtoCodec;

#[derive(Debug, Clone, ProtoCodec)]
pub struct ItemEntry {
    pub name: String,
    /// Block id's < 256 (can be negative); Item id's > 257
    #[endianness(le)]
    pub id: i16,
    pub component_based: bool,
}