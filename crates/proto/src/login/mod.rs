mod start_game;
use crate::connection::{Connection, ConnectionShard};
use crate::error::LoginError;
use crate::login::handshake::handshake;
use crate::login::login::login;
use crate::login::network_settings::network_settings;
use crate::login::packs::packs;
use crate::login::play_status::play_status_login;
use crate::login::provider::{LoginProviderClient, LoginProviderServer};
use crate::login::start_game::start_game;

mod add_actor;
mod handshake;
mod login;
mod network_settings;
mod packs;
mod play_status;
pub mod provider;
mod set_title;

pub async fn login_sequence(
    conn: &mut ConnectionShard,
    mut provider: impl LoginProviderServer,
) -> Result<(), LoginError> {
    network_settings(conn, &mut provider).await?;

    login(conn, &mut provider).await?;
    play_status_login(conn, &mut provider).await?;

    handshake(conn, &mut provider).await?;

    packs(conn, &mut provider).await?;

    start_game(conn, &mut provider).await?;
    Ok(())
}
