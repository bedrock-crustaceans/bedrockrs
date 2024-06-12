use crate::compression::Compression;
use crate::login::provider::LoginProviderServer;

pub struct DefaultLoginProvider {

}

impl DefaultLoginProvider {
    pub fn new() -> Self {Self{

    }}
}

impl LoginProviderServer for DefaultLoginProvider {
    fn compression(&self) -> &Compression {
        &Compression::None
    }

    fn encryption_enabled(&self) -> bool {
        false
    }
}