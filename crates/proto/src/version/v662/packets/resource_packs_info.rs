use std::io::Cursor;
use uuid::Uuid;
use bedrockrs_macros::{gamepacket, ProtoCodec};
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;

#[gamepacket(id = 6)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ResourcePacksInfoPacket {
    pub resource_pack_required: bool,
    pub has_addons: bool,
    pub has_scripts: bool,
    pub forcing_server_packs: bool,
    #[vec_repr(u16)]
    #[vec_endianness(le)]
    pub behavior_packs: Vec<BehaviorPackInfo>,
    #[vec_repr(u16)]
    #[vec_endianness(le)]
    pub resource_packs: Vec<ResourcePackInfo>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub pack_urls: Vec<PackURL>
}

/// Represents information about a Behavior Pack sent over the network.
/// It includes details such as the Behavior Pack's name, description, and version
#[derive(ProtoCodec, Debug, Clone)]
pub struct BehaviorPackInfo {
    /// The UUID represents the unique identifier of the Behavior Pack.
    /// Each downloaded behavior pack must have a distinct UUID so that the client can manage them correctly.
    #[str]
    pub uuid: Uuid,
    /// Specifies the Behavior Pack's version
    /// The client will cache Behavior Packs from the server as long as they have the same version.
    /// If a behavior pack with a different version is sent, the client will be required to re-download it
    pub version: String,
    /// Size refers to the total number of bytes occupied by the Behavior Pack.
    /// This value represents the size of the compressed archive (ZIP)
    #[endianness(le)]
    pub size: u64,
    pub content_key: String,
    pub sub_pack: String,
    pub content_identity: String,
    pub has_scripts: bool,
}


/// Represents information about a Resource Pack sent over the network.
/// It includes details such as the Resource Pack's name, description, and version
#[derive(ProtoCodec, Debug, Clone)]
pub struct ResourcePackInfo {
    /// The UUID represents the unique identifier of the Resource Pack.
    /// Each downloaded Resource Pack must have a distinct UUID so that the client can manage them correctly.
    #[str]
    pub uuid: Uuid,
    /// Specifies the Resource Pack's version
    /// The client will cache Resource Packs from the server as long as they have the same version.
    /// If a Resource Pack with a different version is sent, the client will be required to re-download it
    pub version: String,
    /// Size refers to the total number of bytes occupied by the Resource Pack.
    /// This value represents the size of the compressed archive (ZIP)
    #[endianness(le)]
    pub size: u64,
    pub content_key: String,
    pub sub_pack: String,
    pub content_identity: String,
    pub has_scripts: bool,
    pub ray_tracing_capable: bool,
}

/// PackURL refers to a resource pack hosted on an HTTP server,
/// instead of being delivered through the Minecraft protocol
#[derive(Debug, Clone)]
pub struct PackURL {
    /// Specifies the UUID of Pack that can be downloaded at this location
    /// Gets joined together with the version in the format UUID_Version
    /// The client will download the resource pack only if it is not already cached
    pub uuid: Uuid,
    /// Specifies the version of Pack that can be downloaded at this location
    /// Gets joined together with the UUID in the format UUID_Version 
    /// The client will download the resource pack only if it is not already cached
    pub version: String,
    /// Specifies the address from which the resource pack is downloaded
    /// This URL must provide a zip file containing a manifest.json file within a subfolder,
    /// rather than at the root of the zip
    pub url: String
}

impl ProtoCodec for PackURL {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        // Joined together in the format UUID_Version 
        format!("{}_{}", self.uuid, self.version).proto_serialize(stream)?;
        self.url.proto_serialize(stream)?;
        
        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        let uuid_version = String::proto_deserialize(stream)?;
        let url = String::proto_deserialize(stream)?;
        
        let (uuid, version) = uuid_version
            .split_once("_")
            .ok_or(ProtoCodecError::FormatMismatch("Missing underscore between uuid and version in PackURL"))
            .map(|(u, v)| (u, v.to_string()))?;
    
        let uuid = Uuid::parse_str(uuid)?;
        
        Ok(Self{ uuid, version, url })
    }

    fn get_size_prediction(&self) -> usize {
        self.uuid.get_size_prediction() + self.version.get_size_prediction() + self.url.get_size_prediction()
    }
}
