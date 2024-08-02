#[derive(Debug, Clone)]
pub enum TextMessageData {
    Raw(String),
    Chat {
        player_name: String,
        message: String,
    },
    Translate {
        message: String,
        parameters: Vec<String>,
    },
    Popup {
        message: String,
        parameters: Vec<String>,
    },
    JukeboxPopup {
        message: String,
        parameters: Vec<String>,
    },
    Tip(String),
    SystemMessage(String),
    Whisper {
        player_name: String,
        message: String,
    },
    Announcement {
        player_name: String,
        message: String,
    },
    TextObjectWhisper(String),
    TextObject(String),
    TextObjectAnnouncement(String),
}
