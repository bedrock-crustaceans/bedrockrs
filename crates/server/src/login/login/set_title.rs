use crate::connection::ConnectionShard;
use crate::error::LoginError;
use crate::gamepackets::GamePackets;
use crate::login::provider::LoginProvider;
use crate::packets::set_title::SetTitlePacket;
use crate::types::title_type::TitleType;
use xuid::Xuid;

pub async fn set_title(
    conn: &mut ConnectionShard,
    provider: &mut impl LoginProvider,
) -> Result<(), LoginError> {
    //////////////////////////////////////
    //  Set Title
    //////////////////////////////////////

    let set_title = SetTitlePacket {
        title_type: TitleType::Title,
        title_text: String::from("hello_text"),
        fade_in_time: 500,
        stay_time: 500,
        fade_out_time: 500,
        xuid: Xuid::from(123456789),
        platform_online_id: String::from("hello_platform_online_id"),
    };

    conn.send(GamePackets::SetTitle(set_title)).await?;
    conn.flush().await?;

    Ok(())
}
