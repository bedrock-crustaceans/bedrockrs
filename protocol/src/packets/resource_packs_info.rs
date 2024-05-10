use std::io::Cursor;

use bedrock_core::types::u16le;
use serialize::error::{DeserilizationError, SerilizationError};
use serialize::proto::de::MCProtoDeserialize;
use serialize::proto::ser::MCProtoSerialize;

use crate::types::pack_info_behavior::BehaviorPackInfoType;
use crate::types::pack_info_resource::ResourcePackInfoType;
use crate::types::pack_url::PackURL;

#[derive(Debug)]
pub struct ResourcePacksInfoPacket {
    pub resource_pack_required: bool,
    pub has_addon_packs: bool,
    pub has_scripts: bool,
    pub force_server_packs_enabled: bool,
    pub behavior_packs: Vec<BehaviorPackInfoType>,
    pub resource_packs: Vec<ResourcePackInfoType>,
    pub cdn_urls: Vec<PackURL>,
}

impl MCProtoSerialize for ResourcePacksInfoPacket {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
        where
            Self: Sized,
    {
        // Serialize resource_pack_required as a bool
        match self.resource_pack_required.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        // Serialize has_addon_packs as a bool
        match self.has_addon_packs.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        // Serialize has_scripts as a bool
        match self.has_scripts.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        // Serialize force_server_packs_enabled as a bool
        match self.force_server_packs_enabled.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        // Write length of behavior packs as an u16le
        match u16le(self.behavior_packs.len() as u16).proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        // Write every behavior pack
        for behavior_pack in &self.behavior_packs {
            match behavior_pack.proto_serialize(buf) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }

        // Write length of resource packs as an u16le
        match u16le(self.resource_packs.len() as u16).proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        // Write every resource pack
        for behavior_pack in &self.resource_packs {
            match behavior_pack.proto_serialize(buf) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }

        // Write all cdn urls
        match self.cdn_urls.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        Ok(())
    }
}

impl MCProtoDeserialize for ResourcePacksInfoPacket {
    fn proto_deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
        where
            Self: Sized,
    {
        // TODO: Add this
        todo!()
    }
}
