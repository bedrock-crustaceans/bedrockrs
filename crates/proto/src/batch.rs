use std::io::Cursor;
use crate::compression::Compression;
use crate::encryption::Encryption;
use crate::helper::ProtoHelper;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::sub_client::SubClientID;
use bedrockrs_proto_core::GamePacketsAll;

pub fn batch_gamepackets<T: ProtoHelper>(gamepackets: &[T::GamePacketType], compression: &Option<Compression>, encryption: &mut Option<Encryption>) -> Result<Vec<u8>, ProtoCodecError> {
    let gamepacket_stream_size = gamepackets
        .iter()
        .map(T::GamePacketType::get_size_prediction)
        .sum::<usize>();

    let mut gamepacket_stream = Vec::with_capacity(gamepacket_stream_size);

    // Batch all gamepackets together
    gamepackets.iter().try_for_each(|gamepacket| {
        gamepacket.pk_serialize(
            &mut gamepacket_stream,
            SubClientID::PrimaryClient,
            SubClientID::PrimaryClient,
        )
    })?;

    // Compress the stream with the given optional Compression
    if let Some(compression) = compression {
        gamepacket_stream = compression.compress(gamepacket_stream)?;
    }

    // Encrypt the stream with the given optional Encryption
    if let Some(encryption) = encryption {
        gamepacket_stream = encryption.encrypt(gamepacket_stream)?;
    }
    
    Ok(gamepacket_stream)
}

pub fn separate_gamepackets<T: ProtoHelper>(mut gamepacket_stream: Vec<u8>, compression: &Option<Compression>, encryption: &mut Option<Encryption>) -> Result<Vec<T::GamePacketType>, ProtoCodecError> {
    // Decrypt the stream with the given optional Encryption
    if let Some(encryption) = &mut encryption {
        gamepacket_stream = encryption.decrypt(gamepacket_stream)?;
    }

    // Decompress the stream with the given optional Compression
    if let Some(compression) = &compression {
        gamepacket_stream = compression.decompress(gamepacket_stream)?;
    }

    let mut gamepacket_stream = Cursor::new(gamepacket_stream.as_slice());
    let mut gamepackets = vec![];

    loop {
        if gamepacket_stream.position() == gamepacket_stream.get_ref().len() as u64 {
            break;
        }

        gamepackets.push(T::GamePacketType::pk_deserialize(&mut gamepacket_stream)?.0);
    }

    Ok(gamepackets)
}


