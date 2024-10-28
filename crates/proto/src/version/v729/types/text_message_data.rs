use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum TextMessageData {
    Raw {
        localize: bool,
        message: String,
    } = 0,
    Chat {
        localize: bool,
        player_name: String,
        message: String,
    } = 1,
    Translate {
        localize: bool,
        message: String,
        #[vec_repr(u32)]
        #[vec_endianness(var)]
        parameters: Vec<String>,
    } = 2,
    Popup {
        localize: bool,
        message: String,
        #[vec_repr(u32)]
        #[vec_endianness(var)]
        parameters: Vec<String>,
    } = 3,
    JukeboxPopup {
        localize: bool,
        message: String,
        #[vec_repr(u32)]
        #[vec_endianness(var)]
        parameters: Vec<String>,
    } = 4,
    Tip {
        localize: bool,
        message: String,
    } = 5,
    SystemMessage {
        localize: bool,
        message: String,
    } = 6,
    Whisper {
        localize: bool,
        player_name: String,
        message: String,
    } = 7,
    Announcement {
        localize: bool,
        player_name: String,
        message: String,
    } = 8,
    TextObjectWhisper {
        localize: bool,
        message: String,
    } = 9,
    TextObject {
        localize: bool,
        message: String,
    } = 10,
    TextObjectAnnouncement {
        localize: bool,
        message: String,
    } = 11,
}
