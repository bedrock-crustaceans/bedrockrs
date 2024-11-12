use bedrockrs_proto::connection::Connection;
use crate::connection::ConnectionShard;
use crate::error::LoginError;
use crate::login::handshake::handshake;
use crate::login::login::login;
use crate::login::login::provider::LoginProvider;
use crate::login::network_settings::network_settings;
use crate::login::packs::packs;
use crate::login::play_status::play_status_login;
use crate::login::provider::LoginProvider;
use crate::login::start_game::start_game;

mod add_actor;
mod handshake;
mod login;
mod network_settings;
mod packs;
mod play_status;
pub mod provider;
mod set_title;
mod start_game;

macro_rules! handle_login_status {
    ($provider:ident, $handler:ident, $packet:ident) => {
        match $provider.$handler(&mut $packet) {
            LoginProviderStatus::ContinueLogin => {}
            LoginProviderStatus::AbortLogin { reason, disconnect_reason } => {
                return Err(LoginError::Abort { reason });
            }
        };
    };
}

pub async fn login_sequence(
    conn: &mut Connection,
    mut provider: impl LoginProvider,
) -> Result<(), LoginError> {
    network_settings(conn, &mut provider).await?;

    login(conn, &mut provider).await?;
    play_status_login(conn, &mut provider).await?;

    handshake(conn, &mut provider).await?;

    packs(conn, &mut provider).await?;

    start_game(conn, &mut provider).await?;

    Ok(())
}
