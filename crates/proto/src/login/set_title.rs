use bedrockrs_core::int::VAR;

use crate::connection::ConnectionShard;
use crate::error::LoginError;
use crate::gamepacket::GamePackets;
use crate::login::provider::{LoginProviderServer, LoginProviderStatus};
use crate::packets::play_status::PlayStatusPacket;
use crate::packets::set_title_packet::SetTitlePacket;
use crate::types::play_status::PlayStatusType;
use crate::types::title_type::TitleType;

pub async fn set_title(
    conn: &mut ConnectionShard,
    provider: &mut impl LoginProviderServer,
) -> Result<(), LoginError> {
    //////////////////////////////////////
    //  Set Title
    //////////////////////////////////////

    let set_title = SetTitlePacket {
        title_type: TitleType::Title,
        title_text: String::from("hello_text"),
        fade_in_time: VAR::new(500),
        stay_time: VAR::new(500),
        fade_out_time: VAR::new(500),
        xuid: String::from("hello_xuid"),
        platform_online_id: String::from("hello_platform_online_id"),
    };

    match conn.send(GamePackets::SetTitle(set_title)).await {
        Ok(_) => {}
        Err(e) => return Err(LoginError::ConnectionError(e)),
    }

    match conn.flush().await {
        Ok(_) => {}
        Err(e) => return Err(LoginError::ConnectionError(e)),
    };

    Ok(())
}
