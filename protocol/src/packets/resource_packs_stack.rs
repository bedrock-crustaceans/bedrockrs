use bedrock_core::types::u16le;
use serialize::error::SerilizationError;
use serialize::proto::ser::MCProtoSerialize;
use crate::types::pack_info_behavior::BehaviorPackInfoType;
use crate::types::pack_info_resource::ResourcePackInfoType;
use crate::types::pack_url::PackURL;

pub struct ResourcePacksStackPacket {
    texture_pack_required: bool,
    has_addon_packs: bool,
    has_scripts: bool,
    behavior_packs: Vec<BehaviorPackInfoType>,
    resource_packs: Vec<ResourcePackInfoType>,
    cdn_urls: Vec<PackURL>,
}

impl MCProtoSerialize for ResourcePacksStackPacket {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError> where Self: Sized {
        // Serialize resource_pack_required as a bool
        match self.texture_pack_required.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => { return Err(e) }
        }

        // Serialize has_addon_packs as a bool
        match self.has_addon_packs.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => { return Err(e) }
        }

        // Serialize has_scripts as a bool
        match self.has_scripts.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => { return Err(e) }
        }

        // Write length of behavior packs as an u16le
        match u16le(self.behavior_packs.len() as u16).proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => { return Err(e) }
        }

        // Write every behavior pack
        for behavior_pack in &self.behavior_packs {
            match behavior_pack.proto_serialize(buf) {
                Ok(_) => {}
                Err(e) => { return Err(e) }
            }
        }

        // Write length of resource packs as an u16le
        match u16le(self.resource_packs.len() as u16).proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => { return Err(e) }
        }

        // Write every resource pack
        for behavior_pack in &self.resource_packs {
            match behavior_pack.proto_serialize(buf) {
                Ok(_) => {}
                Err(e) => { return Err(e) }
            }
        }

        // Write all cdn urls
        match self.cdn_urls.proto_serialize(buf){
            Ok(_) => {}
            Err(e) => { return Err(e) }
        }

        Ok(())
    }
}