use crate::version::v662::enums::MolangVersion;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub struct InternalItemDescriptor {
    pub full_name: String,
    #[endianess(le)]
    pub aux_value: u16
}

#[derive(ProtoCodec)]
pub struct MolangDescriptor {
    pub full_name: String,
    pub molang_version: MolangVersion
}

#[derive(ProtoCodec)]
pub struct ItemTagDescriptor {
    pub item_tag: String
}

#[derive(ProtoCodec)]
pub struct DeferredDescriptor {
    pub full_name: String,
    #[endianness(le)]
    pub aux_value: u16
}

#[derive(ProtoCodec)]
pub struct BaseDescription {
    pub internal_item_descriptor: InternalItemDescriptor,
    pub molang_descriptor: MolangDescriptor,
    pub item_tag_descriptor: ItemTagDescriptor,
    pub deferred_descriptor: DeferredDescriptor
}