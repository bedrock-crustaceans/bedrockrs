use packs::behavior::behavior_pack::BehaviorPack;
use packs::resource::resource_pack::ResourcePack;

pub enum LoginProviderPacks {
    CDN {
        behavior_packs: Vec<BehaviorPack>,
        resource_packs: Vec<ResourcePack>,
    },
    DirectNetworkTransfer {
        behavior_packs: Vec<BehaviorPack>,
        resource_packs: Vec<ResourcePack>,
    },
}
