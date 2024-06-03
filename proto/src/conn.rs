use std::io::{Cursor, Error, Write};
use bedrock_core::stream::read::ByteStreamRead;
use bedrock_core::stream::write::ByteStreamWrite;
use tokio::io::AsyncWriteExt;

use crate::compression::Compression;

use crate::encryption::Encryption;
use crate::error::{CompressionError, ConnectionError};
use crate::gamepacket::GamePacket;
use crate::transport_layer::TransportLayerConn;

pub struct Conn {
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

impl Conn {
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

                match compressed_stream.write_u8(compression.id_u8()) {
                    Ok(_) => {}
                    Err(e) => return Err(ConnectionError::IOError(e)),
                };

                if compression.compression_needed() && pk_stream.len() as u16 > compression.threshold() {
                    match compression.compress(&pk_stream.freeze(), &mut compressed_stream) {
                        Ok(_) => { }
                        Err(e) => { return Err(ConnectionError::CompressError(e)) }
                    };
                }
                else {
                    match compressed_stream.write(pk_stream.as_slice()) {
                        Ok(_) => {}
                        Err(e) => { return Err(ConnectionError::IOError(e)) }
                    }
                };

                compressed_stream
            }
            // If no compression is set zero copy the packet stream
            None => {
                pk_stream
            }
        };

        // Encrypt the compressed data
        let encrypted_stream = match &self.encryption {
            Some(encryption) => {
                todo!("Encrypt the data (after compression)")
            }
            None => {
                compressed_stream
            }
        };

        // Send the data
        match self.connection.send(&encrypted_stream.freeze()).await {
            Ok(_) => {}
            Err(e) => return Err(ConnectionError::TransportError(e)),
        }

        Ok(())
    }

    pub async fn recv_gamepackets(&mut self) -> Result<Vec<GamePacket>, ConnectionError> {
        // Receive data and turn it into cursor
        let mut stream = match self.connection.recv().await {
            Ok(v) => v,
            Err(e) => return Err(ConnectionError::TransportError(e)),
        };

        let mut decrypted_stream = match &self.encryption {
            Some(encryption) => {
                todo!("Decrypt the data (before decompression)")
            }
            None => { stream }
        };

        // Decompress data
        let mut decompressed_stream = match &self.compression {
            Some(compression) => {
                match decrypted_stream.read_u8() {
                    Ok(v) => { if v != compression.id_u8() {

                    }}
                    Err(_) => {}
                };

                let mut decompressed_stream = ByteStreamWrite::new();

                match compression.decompress(&mut decrypted_stream, &mut decompressed_stream) {
                    Ok(_) => {}
                    Err(e) => { return Err(ConnectionError::CompressError(e)) }
                };

                decompressed_stream.freeze()
            }
            None => { decrypted_stream }
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
            if decompressed_stream.position() == decompressed_stream.len() as u64 {
                break 'gamepacket_read;
            }
        }

        Ok(gamepackets)
    }
}
