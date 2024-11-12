use bedrockrs_addon::behavior::BehaviorPack;
use bedrockrs_addon::resource::ResourcePack;

use crate::types::pack_url::PackURL;

pub enum LoginProviderPacks {
    CDN {
        behavior_packs: Vec<BehaviorPack>,
        resource_packs: Vec<ResourcePack>,
        cdn_urls: Vec<PackURL>,
    },
    DirectNetworkTransfer {
        behavior_packs: Vec<BehaviorPack>,
        resource_packs: Vec<ResourcePack>,
    },
}
