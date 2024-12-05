use bedrockrs_macros::{gamepacket, ProtoCodec};

#[derive(ProtoCodec)]
struct FeaturesDataListEntry {
    pub feature_name: String,
    pub binary_json_output: String,
}

#[gamepacket(id = 191)]
#[derive(ProtoCodec)]
pub struct FeatureRegistryPacket {
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub features_data_list: Vec<FeaturesDataListEntry>
}