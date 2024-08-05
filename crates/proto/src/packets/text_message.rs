use crate::types::text_message_data::TextMessageData;
use bedrockrs_core::int::VAR;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;
use std::io::Cursor;

#[derive(Debug, Clone)]
pub struct TextMessagePacket {
    pub message_type: TextMessageData,
    pub localize: bool,
    pub sender_xuid: String,
    pub platform_id: String,
    pub filtered_message: String,
}

impl ProtoCodec for TextMessagePacket {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        let message_type: u8 = match self.message_type {
            TextMessageData::Raw(_) => 0,
            TextMessageData::Chat { .. } => 1,
            TextMessageData::Translate { .. } => 2,
            TextMessageData::Popup { .. } => 3,
            TextMessageData::JukeboxPopup { .. } => 4,
            TextMessageData::Tip(_) => 5,
            TextMessageData::SystemMessage(_) => 6,
            TextMessageData::Whisper { .. } => 7,
            TextMessageData::Announcement { .. } => 8,
            TextMessageData::TextObjectWhisper(_) => 9,
            TextMessageData::TextObject(_) => 10,
            TextMessageData::TextObjectAnnouncement(_) => 11,
        };

        message_type.proto_serialize(stream)?;
        self.localize.proto_serialize(stream)?;

        match &self.message_type {
            TextMessageData::Raw(message) => {
                message.proto_serialize(stream)?;
            }
            TextMessageData::Chat {
                player_name,
                message,
            } => {
                player_name.proto_serialize(stream)?;
                message.proto_serialize(stream)?;
            }
            TextMessageData::Translate {
                message,
                parameters,
            } => {
                message.proto_serialize(stream)?;

                let len = VAR::<u32>::new(match Vec::len(parameters).try_into() {
                    Ok(v) => v,
                    Err(e) => return Err(ProtoCodecError::FromIntError(e.into())),
                });

                len.proto_serialize(stream)?;

                for parameter in parameters {
                    parameter.proto_serialize(stream)?;
                }
            }
            TextMessageData::Popup {
                message,
                parameters,
            } => {
                message.proto_serialize(stream)?;

                let len = VAR::<u32>::new(match Vec::len(&parameters).try_into() {
                    Ok(v) => v,
                    Err(e) => return Err(ProtoCodecError::FromIntError(e.into())),
                });

                len.proto_serialize(stream)?;

                for parameter in parameters {
                    parameter.proto_serialize(stream)?;
                }
            }
            TextMessageData::JukeboxPopup {
                message,
                parameters,
            } => {
                message.proto_serialize(stream)?;

                let len = VAR::<u32>::new(match Vec::len(&parameters).try_into() {
                    Ok(v) => v,
                    Err(e) => return Err(ProtoCodecError::FromIntError(e.into())),
                });

                len.proto_serialize(stream)?;

                for parameter in parameters {
                    parameter.proto_serialize(stream)?;
                }
            }
            TextMessageData::Tip(message) => {
                message.proto_serialize(stream)?;
            }
            TextMessageData::SystemMessage(message) => {
                message.proto_serialize(stream)?;
            }
            TextMessageData::Whisper {
                player_name,
                message,
            } => {
                player_name.proto_serialize(stream)?;
                message.proto_serialize(stream)?;
            }
            TextMessageData::Announcement {
                player_name,
                message,
            } => {
                player_name.proto_serialize(stream)?;
                message.proto_serialize(stream)?;
            }
            TextMessageData::TextObjectWhisper(message) => {
                message.proto_serialize(stream)?;
            }
            TextMessageData::TextObject(message) => {
                message.proto_serialize(stream)?;
            }
            TextMessageData::TextObjectAnnouncement(message) => {
                message.proto_serialize(stream)?;
            }
        };

        self.sender_xuid.proto_serialize(stream)?;
        self.platform_id.proto_serialize(stream)?;
        self.filtered_message.proto_serialize(stream)?;

        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        let message_type = u8::proto_deserialize(stream)?;
        let localize = bool::proto_deserialize(stream)?;

        let message_type = match message_type {
            0 => TextMessageData::Raw(String::proto_deserialize(stream)?),
            1 => TextMessageData::Chat {
                player_name: String::proto_deserialize(stream)?,
                message: String::proto_deserialize(stream)?,
            },
            2 => {
                let message = String::proto_deserialize(stream)?;

                let len = VAR::<u32>::proto_deserialize(stream)?.into_inner();
                let mut parameters = Vec::with_capacity(match len.try_into() {
                    Ok(v) => v,
                    Err(e) => return Err(ProtoCodecError::FromIntError(e.into())),
                });

                for _ in 0..len {
                    parameters.push(String::proto_deserialize(stream)?);
                }

                TextMessageData::Translate {
                    message,
                    parameters,
                }
            }
            3 => {
                let message = String::proto_deserialize(stream)?;

                let len = VAR::<u32>::proto_deserialize(stream)?.into_inner();
                let mut parameters = Vec::with_capacity(match len.try_into() {
                    Ok(v) => v,
                    Err(e) => return Err(ProtoCodecError::FromIntError(e.into())),
                });

                for _ in 0..len {
                    parameters.push(String::proto_deserialize(stream)?);
                }

                TextMessageData::Popup {
                    message,
                    parameters,
                }
            }
            4 => {
                let message = String::proto_deserialize(stream)?;

                let len = VAR::<u32>::proto_deserialize(stream)?.into_inner();
                let mut parameters = Vec::with_capacity(match len.try_into() {
                    Ok(v) => v,
                    Err(e) => return Err(ProtoCodecError::FromIntError(e.into())),
                });

                for _ in 0..len {
                    parameters.push(String::proto_deserialize(stream)?);
                }

                TextMessageData::JukeboxPopup {
                    message,
                    parameters,
                }
            }
            5 => TextMessageData::Tip(String::proto_deserialize(stream)?),
            6 => TextMessageData::SystemMessage(String::proto_deserialize(stream)?),
            7 => TextMessageData::Whisper {
                player_name: String::proto_deserialize(stream)?,
                message: String::proto_deserialize(stream)?,
            },
            8 => TextMessageData::Announcement {
                player_name: String::proto_deserialize(stream)?,
                message: String::proto_deserialize(stream)?,
            },
            9 => TextMessageData::TextObjectWhisper(String::proto_deserialize(stream)?),
            10 => TextMessageData::TextObject(String::proto_deserialize(stream)?),
            11 => TextMessageData::TextObjectAnnouncement(String::proto_deserialize(stream)?),
            other => {
                return Err(ProtoCodecError::InvalidEnumID(
                    format!("{other:?}"),
                    String::from("TextMessageData"),
                ));
            }
        };

        let sender_xuid = String::proto_deserialize(stream)?;
        let platform_id = String::proto_deserialize(stream)?;
        let filtered_message = String::proto_deserialize(stream)?;

        Ok(Self {
            message_type,
            localize,
            sender_xuid,
            platform_id,
            filtered_message,
        })
    }
}
