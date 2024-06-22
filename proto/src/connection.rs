use std::future::Future;
use std::io::{Cursor, Write};
use std::sync::Arc;
use std::time::Duration;

use bedrock_core::stream::write::ByteStreamWrite;
use bedrock_core::LE;
use tokio::select;
use tokio::sync::{broadcast, mpsc, watch};
use tokio::time::interval;

use crate::compression::Compression;
use crate::encryption::Encryption;
use crate::error::ConnectionError;
use crate::gamepacket::GamePacket;
use crate::transport_layer::TransportLayerConnection;

pub struct Connection {
    /// Represents the connections internal transport layer, this allows using different
    /// transport layers with the client or proxies, this can improve performance.
    connection: TransportLayerConnection,
    /// Represents the connections' compression, the compression gets initialized in the
    /// login process.
    pub compression: Option<Compression>,
    /// Represents the connections encryption, the encryption gets initialized in the
    /// login process, if encryption is allowed.
    pub encryption: Option<Encryption>,
}

impl Connection {
    pub fn new(conn: TransportLayerConnection) -> Self {
        Self {
            connection: conn,
            compression: None,
            encryption: None,
        }
    }

    pub async fn send(&mut self, gamepackets: Vec<GamePacket>) -> Result<(), ConnectionError> {
        let mut pk_stream = ByteStreamWrite::new();

        // Batch all game packets together
        for game_packet in gamepackets {
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
                    Err(e) => return Err(ConnectionError::IOError(Arc::new(e))),
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
                            return Err(ConnectionError::IOError(Arc::new(e)));
                        }
                    }
                };

                compressed_stream
            }
            // If no compression is set none copy the packet stream
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

    pub async fn send_raw(&mut self, data: &Vec<u8>) -> Result<(), ConnectionError> {
        // Send the data
        match self.connection.send(&Cursor::new(data)).await {
            Ok(_) => {}
            Err(e) => return Err(ConnectionError::TransportError(e)),
        }

        Ok(())
    }

    pub async fn recv(&mut self) -> Result<Vec<GamePacket>, ConnectionError> {
        let mut stream = ByteStreamWrite::new();

        // Receive data and turn it into cursor
        match self.connection.recv(&mut stream).await {
            Ok(_) => {}
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

    pub async fn recv_raw(&mut self) -> Result<Vec<u8>, ConnectionError> {
        let mut stream = vec![];

        // Send the data
        match self.connection.recv(&mut stream).await {
            Ok(_) => {}
            Err(e) => return Err(ConnectionError::TransportError(e)),
        }

        Ok(stream)
    }

    pub async fn into_shard(
        mut self,
        flush_interval: Duration,
        packet_buffer_size: usize,
    ) -> ConnectionShard {
        let (shard_pk_sender, mut task_pk_receiver) =
            broadcast::channel::<GamePacket>(packet_buffer_size);
        let (task_pk_sender, shard_pk_receiver) =
            broadcast::channel::<Result<GamePacket, ConnectionError>>(packet_buffer_size);

        let (shard_close_sender, task_close_receiver) = watch::channel::<()>(());

        let (shard_compression_sender, mut task_compression_receiver) =
            watch::channel::<Option<Compression>>(self.compression.clone());
        let (shard_encryption_sender, mut task_encryption_receiver) =
            watch::channel::<Option<Encryption>>(self.encryption.clone());

        let (shard_compression_request_sender, mut task_compression_request_receiver) =
            watch::channel(());
        let (mut task_compression_sender, shard_compression_receiver) =
            watch::channel(self.compression.clone());

        let (shard_encryption_request_sender, mut task_encryption_request_receiver) =
            watch::channel(());
        let (mut task_encryption_sender, shard_encryption_receiver) =
            watch::channel(self.encryption.clone());

        tokio::spawn(async move {
            let mut flush_interval = interval(flush_interval);
            let mut send_buffer = vec![];

            loop {
                select! {
                    res = self.recv() => {
                        match res {
                            Ok(pks) => {
                                for pk in pks {
                                    task_pk_sender.send(Ok(pk)).expect("TODO: panic message");
                                }
                            }
                            Err(e) => {
                                task_pk_sender.send(Err(e)).expect("TODO: panic message");
                            }
                        }
                    }
                    res = task_pk_receiver.recv() => {
                        match res {
                            Ok(pk) => { send_buffer.push(pk); }
                            Err(_) => {}
                        };
                    }
                    _ = task_compression_receiver.changed() => {
                        self.compression = task_compression_receiver.borrow_and_update().to_owned();
                    }
                    _ = task_encryption_receiver.changed() => {
                        self.encryption = task_encryption_receiver.borrow_and_update().to_owned();
                    }
                    _ = task_compression_request_receiver.changed() => {
                        task_compression_sender.send(self.compression.clone()).expect("TODO: panic message");
                    }
                    _ = task_encryption_request_receiver.changed() => {
                        task_encryption_sender.send(self.encryption.clone()).expect("TODO: panic message");
                    }
                    _ = flush_interval.tick() => {
                        self.send(send_buffer).await.expect("TODO: panic message");
                        send_buffer = vec![];
                    }
                }
            }
        });

        ConnectionShard {
            pk_sender: shard_pk_sender,
            pk_receiver: shard_pk_receiver,
            close_sender: shard_close_sender,
            compression_sender: shard_compression_sender,
            encryption_sender: shard_encryption_sender,
            compression_request_sender: shard_compression_request_sender,
            compression_receiver: shard_compression_receiver,
            encryption_request_sender: shard_encryption_request_sender,
            encryption_receiver: shard_encryption_receiver,
        }
    }
}

pub struct ConnectionShard {
    pk_sender: broadcast::Sender<GamePacket>,
    pk_receiver: broadcast::Receiver<Result<GamePacket, ConnectionError>>,
    close_sender: watch::Sender<()>,
    compression_sender: watch::Sender<Option<Compression>>,
    encryption_sender: watch::Sender<Option<Encryption>>,
    compression_request_sender: watch::Sender<()>,
    compression_receiver: watch::Receiver<Option<Compression>>,
    encryption_request_sender: watch::Sender<()>,
    encryption_receiver: watch::Receiver<Option<Encryption>>,
}

impl ConnectionShard {
    pub async fn send(&mut self, pk: GamePacket) -> Result<(), ConnectionError> {
        match self.pk_sender.send(pk) {
            Ok(_) => Ok(()),
            Err(_) => Err(ConnectionError::ConnectionClosed),
        }
    }

    pub async fn recv(&mut self) -> Result<GamePacket, ConnectionError> {
        match self.pk_receiver.recv().await {
            Ok(pk) => pk,
            Err(_) => Err(ConnectionError::ConnectionClosed),
        }
    }

    pub fn close(&mut self) {
        match self.close_sender.send(()) {
            Ok(_) => { /* has been closed successfully */ }
            Err(_) => { /* has already been closed */ }
        }
    }

    pub async fn set_compression(
        &mut self,
        compression: Option<Compression>,
    ) -> Result<(), ConnectionError> {
        match self.compression_sender.send(compression) {
            Ok(_) => Ok(()),
            Err(_) => Err(ConnectionError::ConnectionClosed),
        }
    }

    pub async fn get_compression(&mut self) -> Result<Option<Compression>, ConnectionError> {
        match self.compression_request_sender.send(()) {
            Ok(_) => {}
            Err(_) => return Err(ConnectionError::ConnectionClosed),
        };

        match self.compression_receiver.changed().await {
            Ok(_) => Ok(self
                .compression_receiver
                .borrow_and_update()
                .clone()
                .to_owned()),
            Err(_) => Err(ConnectionError::ConnectionClosed),
        }
    }

    pub async fn set_encryption(
        &mut self,
        encryption: Option<Encryption>,
    ) -> Result<(), ConnectionError> {
        match self.encryption_sender.send(encryption) {
            Ok(_) => Ok(()),
            Err(_) => Err(ConnectionError::ConnectionClosed),
        }
    }

    pub async fn get_encryption(&mut self) -> Result<Option<Encryption>, ConnectionError> {
        match self.encryption_request_sender.send(()) {
            Ok(_) => {}
            Err(_) => return Err(ConnectionError::ConnectionClosed),
        };

        match self.encryption_receiver.changed().await {
            Ok(_) => Ok(self
                .encryption_receiver
                .borrow_and_update()
                .clone()
                .to_owned()),
            Err(_) => Err(ConnectionError::ConnectionClosed),
        }
    }
}

impl Clone for ConnectionShard {
    fn clone(&self) -> Self {
        Self {
            pk_sender: self.pk_sender.clone(),
            pk_receiver: self.pk_receiver.resubscribe(),
            close_sender: self.close_sender.clone(),
            compression_sender: self.compression_sender.clone(),
            encryption_sender: self.encryption_sender.clone(),
            compression_request_sender: self.compression_request_sender.clone(),
            compression_receiver: self.compression_receiver.clone(),
            encryption_request_sender: self.compression_request_sender.clone(),
            encryption_receiver: self.encryption_receiver.clone(),
        }
    }
}
