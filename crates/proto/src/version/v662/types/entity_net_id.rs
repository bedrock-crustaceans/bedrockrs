use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub struct EntityNetID {
    #[endianness(var)]
    pub raw_entity_id: u32
}