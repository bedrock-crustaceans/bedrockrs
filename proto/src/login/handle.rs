use crate::connection::{Connection, ConnectionShard};
use crate::error::LoginError;
use crate::login::handshake::handshake;
use crate::login::login::login;
use crate::login::network_settings::network_settings;
use crate::login::packs::packs;
use crate::login::provider::{LoginProviderClient, LoginProviderServer};
use crate::login::start_game::play_status_login;

pub async fn login_to_server(
    conn: &mut ConnectionShard,
    mut provider: impl LoginProviderServer,
) -> Result<(), LoginError> {
    network_settings(conn, &mut provider).await?;

    login(conn, &mut provider).await?;
    play_status_login(conn, &mut provider).await?;

    handshake(conn, &mut provider).await?;

    packs(conn, &mut provider).await?;

    Ok(())
}

pub async fn login_to_client(
    conn: &mut Connection,
    provider: impl LoginProviderClient,
) -> Result<(), LoginError> {
    todo!()
}
