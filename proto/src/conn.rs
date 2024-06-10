use std::io::{Cursor, Write};

use bedrock_core::stream::write::ByteStreamWrite;
use bedrock_core::LE;

use crate::compression::Compression;
use crate::encryption::Encryption;
use crate::error::ConnectionError;
use crate::gamepacket::GamePacket;
use crate::transport_layer::TransportLayerConn;

pub struct Connection {
    /// Represents the connections internal transport layer, this allows using different
    /// transport layers with the client or proxies, this can improve performance.
    connection: TransportLayerConn,
    /// Represents the connections' compression, the compression gets initialized in the
    /// login process.
    pub compression: Option<Compression>,
    /// Represents the connections encryption, the encryption gets initialized in the
    /// login process, if encryption is allowed.
    pub encryption: Option<Encryption>,
}

impl Connection {
    pub fn new(conn: TransportLayerConn) -> Self {
        Self {
            connection: conn,
            compression: None,
            encryption: None,
        }
    }

    pub async fn send_gamepackets(
        &mut self,
        game_packets: Vec<GamePacket>,
    ) -> Result<(), ConnectionError> {
        let mut pk_stream = ByteStreamWrite::new();

        // Batch all game packets together
        for game_packet in game_packets {
            // Write a game packet
            match game_packet.pk_serialize(&mut pk_stream) {
                Ok(_) => {}
                Err(e) => return Err(ConnectionError::ProtoCodecError(e)),
            }
        }

        // Compress the data depending on compression method
        let compressed_stream = match &self.compression {
            Some(compression) => {
                let mut compressed_stream = ByteStreamWrite::new();

                match LE::<u8>::write(&LE::new(compression.id_u8()), &mut compressed_stream) {
                    Ok(_) => {}
                    Err(e) => return Err(ConnectionError::IOError(e)),
                };

                if compression.compression_needed()
                    && pk_stream.len() as u16 > compression.threshold()
                {
                    match compression.compress(&Cursor::new(&pk_stream), &mut compressed_stream) {
                        Ok(_) => {}
                        Err(e) => {
                            return Err(ConnectionError::CompressError(e));
                        }
                    };
                } else {
                    match compressed_stream.write(pk_stream.as_slice()) {
                        Ok(_) => {}
                        Err(e) => {
                            return Err(ConnectionError::IOError(e));
                        }
                    }
                };

                compressed_stream
            }
            // If no compression is set zero copy the packet stream
            None => pk_stream,
        };

        // Encrypt the compressed data
        let encrypted_stream = match &self.encryption {
            Some(encryption) => {
                todo!("Encrypt the data (after compression)")
            }
            None => compressed_stream,
        };

        // Send the data
        match self.connection.send(&Cursor::new(&encrypted_stream)).await {
            Ok(_) => {}
            Err(e) => return Err(ConnectionError::TransportError(e)),
        }

        Ok(())
    }

    pub async fn recv_gamepackets(&mut self) -> Result<Vec<GamePacket>, ConnectionError> {
        let mut stream = ByteStreamWrite::new();

        // Receive data and turn it into cursor
        match self.connection.recv(&mut stream).await {
            Ok(v) => v,
            Err(e) => return Err(ConnectionError::TransportError(e)),
        };

        let stream = Cursor::new(&stream);

        let mut decrypted_stream = match &self.encryption {
            Some(encryption) => {
                todo!("Decrypt the data (before decompression)")
            }
            None => stream,
        };

        let mut decompressed_stream = ByteStreamWrite::new();

        // Decompress data
        let mut decompressed_stream = match &self.compression {
            Some(compression) => {
                match LE::<u8>::read(&mut decrypted_stream) {
                    Ok(v) => {
                        if v.into_inner() != compression.id_u8() {
                            // TODO: Handle invalid compression method
                        }
                    }
                    Err(_) => {}
                };

                match compression.decompress(&mut decrypted_stream, &mut decompressed_stream) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(ConnectionError::CompressError(e));
                    }
                };

                Cursor::new(&decompressed_stream)
            }
            None => decrypted_stream,
        };

        let mut gamepackets = vec![];

        // Read gamepacket loop
        'gamepacket_read: loop {
            // Deserialize gamepacket
            match GamePacket::pk_deserialize(&mut decompressed_stream) {
                Ok(v) => gamepackets.push(v.0),
                Err(e) => return Err(ConnectionError::ProtoCodecError(e)),
            };

            // Is at the end of batched packet data cursor
            // TODO: Overflow checking
            if decompressed_stream.position() == decompressed_stream.get_ref().len() as u64 {
                break 'gamepacket_read;
            }
        }

        Ok(gamepackets)
    }
}
