use crate::connection::ConnectionShard;
use crate::error::LoginError;
use crate::gamepacket::GamePacket;
use crate::login::provider::{LoginProviderServer, LoginProviderStatus};

pub async fn login(conn: &mut ConnectionShard, provider: &impl LoginProviderServer) -> Result<(), LoginError> {
    //////////////////////////////////////
    // Login Packet
    //////////////////////////////////////

    let mut login = match conn.recv().await {
        Ok(GamePacket::Login(pk)) => pk,
        Ok(other) => {
            return Err(LoginError::FormatError(format!(
                "Expected Login packet, got: {other:?}"
            )))
        }
        Err(e) => return Err(LoginError::ConnError(e)),
    };

    match provider.on_login_pk(&mut login) {
        LoginProviderStatus::ContinueLogin => {}
        LoginProviderStatus::AbortLogin { reason } => {
            return Err(LoginError::Abort { reason });
        }
    };

    if provider.auth_enabled() {
        todo!("impl xbox auth with data from login pk")
    };

    Ok(())
}