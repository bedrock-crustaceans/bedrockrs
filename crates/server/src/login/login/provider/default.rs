use crate::compression::Compression;
use crate::login::provider::packs::LoginProviderPacks;
use crate::login::provider::LoginProvider;

pub struct DefaultLoginProvider {
    packs: LoginProviderPacks,
}

impl DefaultLoginProvider {
    pub fn new() -> Self {
        Self {
            packs: LoginProviderPacks::CDN {
                behavior_packs: vec![],
                resource_packs: vec![],
                cdn_urls: vec![],
            },
        }
    }
}

impl Default for DefaultLoginProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl LoginProvider for DefaultLoginProvider {
    fn compression(&self) -> Compression {
        Compression::None
    }
    fn encryption_enabled(&self) -> bool {
        false
    }

    fn auth_enabled(&self) -> bool {
        false
    }

    fn packs(&self) -> &LoginProviderPacks {
        &self.packs
    }
}
