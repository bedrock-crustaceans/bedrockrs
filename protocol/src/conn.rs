use std::io::{Cursor, Write};

use byteorder::{ReadBytesExt, WriteBytesExt};
use rak_rs::connection::Connection as RakConnection;

use crate::compression::none::NoCompression;
use crate::compression::snappy::SnappyCompression;
use crate::compression::zlib::ZlibCompression;
use crate::compression::{CompressionMethod, CompressionMethods};
use crate::encryption::Encryption;
use crate::error::ConnectionError;
use crate::gamepacket::GamePacket;
use crate::info::RAKNET_GAME_PACKET_ID;

pub struct Connection {
    rak_connection: RakConnection,
    compression: Option<CompressionMethods>,
    encryption: Option<Encryption>,
}

impl Connection {
    pub fn new(conn: RakConnection) -> Self {
        Self {
            rak_connection: conn,
            compression: None,
            encryption: None,
        }
    }

    pub fn set_compression_method(&mut self, compression_method: Option<CompressionMethods>) {
        self.compression = compression_method
    }

    pub fn get_compression_method(&self) -> &Option<CompressionMethods> {
        &self.compression
    }

    pub async fn send_gamepackets(
        &self,
        game_packets: Vec<GamePacket>,
    ) -> Result<(), ConnectionError> {
        let mut buf_pks = vec![];

        // Batch all gamepackets together
        for game_packet in game_packets {
            // Write gamepacket
            match game_packet.pk_serilize(&mut buf_pks) {
                Ok(_) => {}
                Err(e) => return Err(ConnectionError::SerializeError(e)),
            }
        }

        let mut buf = vec![];

        match buf.write_u8(RAKNET_GAME_PACKET_ID) {
            Ok(_) => {}
            Err(_) => return Err(ConnectionError::WriteIOError),
        };

        // Compress the data depending on compression method
        if let Some(compression) = &self.compression {
            match compression {
                CompressionMethods::Zlib(zlib) => {
                    // If the data is more than the compression threshold or equal to it
                    // then compress it, else don't
                    if buf_pks.len() as u64 >= zlib.threshold as u64 {
                        // Write compression id for zlib
                        match buf.write_u8(ZlibCompression::ID_u8) {
                            Ok(_) => {}
                            Err(_) => return Err(ConnectionError::WriteIOError),
                        };

                        // Decompress using zlib
                        buf_pks = match zlib.compress(buf_pks) {
                            Ok(v) => v,
                            Err(e) => return Err(ConnectionError::CompressError(e)),
                        };
                    } else {
                        // Write compression id for no compression
                        match buf.write_u8(NoCompression::ID_u8) {
                            Ok(_) => {}
                            Err(_) => return Err(ConnectionError::WriteIOError),
                        };
                    };
                }
                CompressionMethods::Snappy(snappy) => {
                    // If the data is more than the compression threshold or equal to it
                    // then compress it, else don't
                    if buf_pks.len() as u64 >= snappy.threshold as u64 {
                        // Write compression id for snappy
                        match buf.write_u8(SnappyCompression::ID_u8) {
                            Ok(_) => {}
                            Err(_) => return Err(ConnectionError::WriteIOError),
                        };

                        // Decompress using Snappy
                        buf_pks = match snappy.compress(buf_pks) {
                            Ok(v) => v,
                            Err(e) => return Err(ConnectionError::CompressError(e)),
                        };
                    } else {
                        // Write compression id for no compression
                        match buf.write_u8(NoCompression::ID_u8) {
                            Ok(_) => {}
                            Err(_) => return Err(ConnectionError::WriteIOError),
                        };
                    };
                }
                CompressionMethods::None => {
                    // Write compression id
                    match buf.write_u8(NoCompression::ID_u8) {
                        Ok(_) => {}
                        Err(_) => return Err(ConnectionError::WriteIOError),
                    };

                    // No decompression needed
                }
            }
        }

        // TODO: Encrypt the data (after compression)

        // Write final data into buf
        match buf.write_all(&*buf_pks) {
            Ok(_) => {}
            Err(_) => return Err(ConnectionError::WriteIOError),
        };

        match self.rak_connection.send(&*buf, true).await {
            Ok(_) => {}
            Err(_) => return Err(ConnectionError::RakNetError),
        }

        Ok(())
    }

    pub async fn recv_gamepackets(&mut self) -> Result<Vec<GamePacket>, ConnectionError> {
        // Recvieve data and turn it into cursor
        let mut data = match self.rak_connection.recv().await {
            Ok(v) => Cursor::new(v),
            Err(_) => return Err(ConnectionError::ConnectionClosed),
        };

        // Read Raknet Header
        match data.read_u8() {
            Ok(v) => {
                match v {
                    RAKNET_GAME_PACKET_ID => {}
                    // Invalid Raknet packet header
                    _ => return Err(ConnectionError::InvalidRakNetHeader),
                }
            }
            Err(_) => return Err(ConnectionError::ReadIOError),
        };

        // TODO: Decrypt the data (before decompression)

        // Decompress data depending on compression method
        if let Some(compression) = &self.compression {
            // Read compression algorythm
            let compression_id = match data.read_u8() {
                Ok(v) => v,
                Err(_) => return Err(ConnectionError::ReadIOError),
            };

            let pos = data.position() as usize;

            match compression_id {
                ZlibCompression::ID_u8 => match compression {
                    CompressionMethods::Zlib(zlib) => {
                        data =
                            Cursor::new(match zlib.decompress(data.into_inner()[pos..].to_vec()) {
                                Ok(v) => v,
                                Err(e) => return Err(ConnectionError::CompressError(e)),
                            })
                    }
                    CompressionMethods::Snappy(_) => {
                        return Err(ConnectionError::WrongCompressionMethod)
                    }
                    CompressionMethods::None => {
                        return Err(ConnectionError::WrongCompressionMethod)
                    }
                },
                SnappyCompression::ID_u8 => match compression {
                    CompressionMethods::Zlib(_) => {
                        return Err(ConnectionError::WrongCompressionMethod)
                    }
                    CompressionMethods::Snappy(snappy) => {
                        data = Cursor::new(
                            match snappy.decompress(data.into_inner()[pos..].to_vec()) {
                                Ok(v) => v,
                                Err(e) => return Err(ConnectionError::CompressError(e)),
                            },
                        )
                    }
                    CompressionMethods::None => {
                        return Err(ConnectionError::WrongCompressionMethod)
                    }
                },
                NoCompression::ID_u8 => {}
                other => return Err(ConnectionError::UnknownCompressionMethod(other)),
            }
        }

        let mut gamepackets = vec![];

        // Read gamepacket loop
        'gamepacket_read: loop {
            // Deserialize gamepacket
            match GamePacket::pk_deserialize(&mut data) {
                Ok(v) => gamepackets.push(v),
                Err(e) => return Err(ConnectionError::DeserializeError(e)),
            };

            // Is at end of batched packet data cursor
            if data.position() == data.get_ref().len() as u64 {
                break 'gamepacket_read;
            }
        }

        Ok(gamepackets)
    }
}
