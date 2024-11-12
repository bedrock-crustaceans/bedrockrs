use crate::connection::ConnectionShard;
use crate::error::LoginError;
use crate::gamepackets::GamePackets;
use crate::login::provider::{LoginProvider, LoginProviderStatus};

pub async fn login(
    conn: &mut ConnectionShard,
    provider: &mut impl LoginProvider,
) -> LoginProviderStatus {
    //////////////////////////////////////
    // Login Packet
    //////////////////////////////////////

    let mut login = match conn.recv().await? {
        GamePackets::Login(pk) => pk,
        other => {
            return Err(LoginError::FormatError(format!(
                "Expected Login packet, got: {other:?}"
            )))
        }
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
