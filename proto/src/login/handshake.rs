use crate::connection::ConnectionShard;
use crate::error::LoginError;
use crate::login::provider::LoginProviderServer;

pub async fn handshake(
    conn: &mut ConnectionShard,
    provider: &mut impl LoginProviderServer,
) -> Result<(), LoginError> {
    if !provider.encryption_enabled() {
        return Ok(())
    };

    todo!("impl the handshake")
}
