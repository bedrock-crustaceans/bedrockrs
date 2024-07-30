use bedrockrs_core::int::LE;
use bedrockrs_core::int::VAR;
use bedrockrs_proto_core::ProtoCodec;
use bedrockrs_proto_derive::ProtoCodec;

use crate::types::pack_info_behavior::BehaviorPackInfoType;
use crate::types::pack_info_resource::ResourcePackInfoType;
use crate::types::pack_url::PackURL;

#[derive(Debug, Clone, ProtoCodec)]
pub struct ResourcePacksInfoPacket {
    pub resource_pack_required: bool,
    pub has_addon_packs: bool,
    pub has_scripts: bool,
    pub force_server_packs_enabled: bool,
    #[len_repr(LE::< u16 >)]
    pub behavior_packs: Vec<BehaviorPackInfoType>,
    #[len_repr(LE::< u16 >)]
    pub resource_packs: Vec<ResourcePackInfoType>,
    #[len_repr(VAR::< u32 >)]
    pub cdn_urls: Vec<PackURL>,
}

// impl ProtoCodec for ResourcePacksInfoPacket {
//     fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError>
//     where
//         Self: Sized,
//     {
//         // Serialize resource_pack_required as a Bool
//         match self.resource_pack_required.proto_serialize(stream) {
//             Ok(_) => {}
//             Err(e) => return Err(e),
//         }
//
//         // Serialize has_addon_packs as a Bool
//         match self.has_addon_packs.proto_serialize(stream) {
//             Ok(_) => {}
//             Err(e) => return Err(e),
//         }
//
//         // Serialize has_scripts as a Bool
//         match self.has_scripts.proto_serialize(stream) {
//             Ok(_) => {}
//             Err(e) => return Err(e),
//         }
//
//         // Serialize force_server_packs_enabled as a Bool
//         match self.force_server_packs_enabled.proto_serialize(stream) {
//             Ok(_) => {}
//             Err(e) => return Err(e),
//         }
//
//         let len = match self.behavior_packs.len().try_into() {
//             Ok(v) => v,
//             Err(e) => {
//                 return Err(ProtoCodecError::FromIntError(e));
//             }
//         };
//
//         // Write length of behavior addons as an u16le
//         match LE::<u16>::new(len).proto_serialize(stream) {
//             Ok(_) => {}
//             Err(e) => return Err(e),
//         }
//
//         // Write every behavior pack
//         for behavior_pack in &self.behavior_packs {
//             match behavior_pack.proto_serialize(stream) {
//                 Ok(_) => {}
//                 Err(e) => return Err(e),
//             }
//         }
//
//         let len = match self.resource_packs.len().try_into() {
//             Ok(v) => v,
//             Err(e) => {
//                 return Err(ProtoCodecError::FromIntError(e));
//             }
//         };
//
//         // Write length of resource addons as an u16le
//         match LE::<u16>::new(len).proto_serialize(stream) {
//             Ok(_) => {}
//             Err(e) => return Err(e),
//         }
//
//         // Write every resource pack
//         for behavior_pack in &self.resource_packs {
//             match behavior_pack.proto_serialize(stream) {
//                 Ok(_) => {}
//                 Err(e) => return Err(e),
//             }
//         }
//
//         // Write all cdn urls
//         match self.cdn_urls.proto_serialize(stream) {
//             Ok(_) => {}
//             Err(e) => return Err(e),
//         }
//
//         Ok(())
//     }
//
//     fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError>
//     where
//         Self: Sized,
//     {
//         todo!()
//     }
// }
