use crate::connection::ConnectionShard;
use crate::error::LoginError;
use crate::login::provider::{LoginProvider, LoginProviderStatus};

pub async fn handshake(
    conn: &mut ConnectionShard,
    provider: &mut impl LoginProvider,
) -> LoginProviderStatus {
    
    if !provider.encryption_enabled() {
        return Ok(());
    };

    todo!("impl the handshake")
}
