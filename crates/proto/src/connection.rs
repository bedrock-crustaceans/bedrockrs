use std::io::{Cursor, Write};
use std::sync::Arc;
use std::time::Duration;
use byteorder::WriteBytesExt;
use tokio::select;
use tokio::sync::{broadcast, watch};
use tokio::time::interval;

use crate::compression::Compression;
use crate::encryption::Encryption;
use crate::error::ConnectionError;
use crate::gamepackets::GamePackets;
use crate::sub_client::SubClientID;
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
    pub cache_supported: bool,
}

impl Connection {
    pub fn from_transport_conn(conn: TransportLayerConnection) -> Self {
        Self {
            connection: conn,
            compression: None,
            encryption: None,
            cache_supported: false,
        }
    }

    pub async fn send(&mut self, gamepackets: &[GamePackets]) -> Result<(), ConnectionError> {
        let mut pk_stream = vec![];

        // Batch all gamepackets together
        for game_packet in gamepackets {
            // Write gamepacket
            game_packet
                .pk_serialize(
                    &mut pk_stream,
                    SubClientID::PrimaryClient,
                    SubClientID::PrimaryClient,
                )
                .map_err(ConnectionError::ProtoCodecError)?
        }

        // Compress the data depending on compression method
        let compressed_stream = match &self.compression {
            Some(compression) => {
                let mut compressed_stream = vec![];

                compressed_stream.write_u8(compression.id_u8())?;

                // TODO: Overflow checking
                if compression.needed() && pk_stream.len() as u16 > compression.threshold() {
                    compression
                        .compress(pk_stream.as_slice(), &mut compressed_stream)
                        .map_err(ConnectionError::CompressError)?;
                } else {
                    compressed_stream.write(pk_stream.as_slice())?;
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
        self.connection
            .send(&Cursor::new(&encrypted_stream))
            .await
            .map_err(ConnectionError::TransportError)
    }

    pub async fn send_raw(&mut self, data: &[u8]) -> Result<(), ConnectionError> {
        // Send the data
        match self.connection.send(&Cursor::new(data)).await {
            Ok(_) => {}
            Err(e) => return Err(ConnectionError::TransportError(e)),
        }

        Ok(())
    }

    pub async fn recv(&mut self) -> Result<Vec<GamePackets>, ConnectionError> {
        let mut stream = vec![];

        // Receive data and turn it into cursor
        match self.connection.recv(&mut stream).await {
            Ok(_) => {}
            Err(e) => return Err(ConnectionError::TransportError(e)),
        };

        let stream = Cursor::new(stream.as_slice());

        let mut decrypted_stream = match &self.encryption {
            Some(encryption) => {
                todo!("Decrypt the data (before decompression)")
            }
            None => stream,
        };

        let mut decompressed_stream = vec![];

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

                let pos = decrypted_stream.position() as usize;

                compression
                    .decompress(
                        &decrypted_stream.into_inner()[pos..],
                        &mut decompressed_stream,
                    )
                    .map_err(|e| ConnectionError::CompressError(e))?;

                Cursor::new(decompressed_stream.as_slice())
            }
            None => decrypted_stream,
        };

        let mut gamepackets = vec![];

        // Read gamepacket loop
        'gamepacket_read: loop {
            // Deserialize gamepacket
            match GamePackets::pk_deserialize(&mut decompressed_stream) {
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

    pub async fn close(self) {
        self.connection.close().await;
    }

    pub async fn into_shard(
        mut self,
        flush_interval: Duration,
        packet_buffer_size: usize,
    ) -> ConnectionShard {
        let (shard_pk_sender, mut task_pk_receiver) =
            broadcast::channel::<GamePackets>(packet_buffer_size);
        let (task_pk_sender, shard_pk_receiver) =
            broadcast::channel::<Result<GamePackets, ConnectionError>>(packet_buffer_size);

        let (shard_flush_request_sender, mut task_flush_request_receiver) = watch::channel(());
        let (task_flush_complete_sender, mut shard_flush_complete_receiver) = watch::channel(());

        let (shard_close_sender, mut task_close_receiver) = watch::channel(());

        let (shard_compression_sender, mut task_compression_receiver) =
            watch::channel(self.compression.clone());
        let (shard_compression_request_sender, mut task_compression_request_receiver) =
            watch::channel(());
        let (mut task_compression_sender, shard_compression_receiver) =
            watch::channel(self.compression.clone());

        let (shard_encryption_sender, mut task_encryption_receiver) =
            watch::channel(self.encryption.clone());
        let (shard_encryption_request_sender, mut task_encryption_request_receiver) =
            watch::channel(());
        let (mut task_encryption_sender, shard_encryption_receiver) =
            watch::channel(self.encryption.clone());

        let (shard_cache_supported_sender, mut task_cache_supported_receiver) =
            watch::channel(self.cache_supported.clone());
        let (shard_cache_supported_request_sender, mut task_cache_supported_request_receiver) =
            watch::channel(());
        let (mut task_cache_supported_sender, shard_cache_supported_receiver) =
            watch::channel(self.cache_supported.clone());

        tokio::spawn(async move {
            let mut flush_interval = interval(flush_interval);
            let mut send_buffer = vec![];

            'select_loop: loop {
                select! {
                    _ = task_close_receiver.changed() => {
                        break 'select_loop
                    }
                    res = task_compression_receiver.changed() => {
                        if let Err(_) = res {
                            break 'select_loop
                        }

                        self.compression = task_compression_receiver.borrow_and_update().to_owned();
                    }
                    res = task_encryption_receiver.changed() => {
                        if let Err(_) = res {
                            break 'select_loop
                        }

                        self.encryption = task_encryption_receiver.borrow_and_update().to_owned();
                    }
                    res = task_cache_supported_receiver.changed() => {
                        if let Err(_) = res {
                            break 'select_loop
                        }

                        self.cache_supported = task_cache_supported_receiver.borrow_and_update().to_owned();
                    }
                    res = task_compression_request_receiver.changed() => {
                        if let Err(_) = res {
                            break 'select_loop
                        }

                        if let Err(_) = task_compression_sender.send(self.compression.clone()) {
                            break 'select_loop
                        }
                    }
                    res = task_encryption_request_receiver.changed() => {
                        if let Err(_) = res {
                            break 'select_loop
                        }

                        if let Err(_) = task_encryption_sender.send(self.encryption.clone()) {
                            break 'select_loop
                        }
                    }
                    res = task_cache_supported_request_receiver.changed() => {
                        if let Err(_) = res {
                            break 'select_loop
                        }

                        if let Err(_) = task_cache_supported_sender.send(self.cache_supported.clone()) {
                            break 'select_loop
                        }
                    }
                    res = self.recv() => {
                        match res {
                            Ok(pks) => {
                                for pk in pks {
                                    if task_pk_sender.send(Ok(pk)).is_err() {
                                        break 'select_loop
                                    }
                                }
                            }
                            Err(e) => {
                                if task_pk_sender.send(Err(e)).is_err() {
                                    break 'select_loop
                                }
                            }
                        }
                    }
                    res = task_pk_receiver.recv() => {
                        if res.is_err() {
                            break 'select_loop
                        }

                        match res {
                            Ok(pk) => { send_buffer.push(pk); }
                            Err(e) => { break 'select_loop }
                        };
                    }
                    res = task_flush_request_receiver.changed() => {
                        if res.is_err() {
                            break 'select_loop
                        }

                        if !send_buffer.is_empty() {
                            if let Err(_) = self.send(send_buffer.as_slice()).await {
                                break 'select_loop
                            }

                            if task_flush_complete_sender.send(()).is_err() {
                                break 'select_loop
                            }

                            send_buffer = vec![];
                        }
                    }
                    _ = flush_interval.tick() => {
                        if !send_buffer.is_empty() {
                            if (self.send(send_buffer.as_slice()).await).is_err() {
                                break 'select_loop
                            }

                            send_buffer = vec![];
                        }
                    }
                }
            }

            self.connection.close().await;
        });

        ConnectionShard {
            pk_sender: shard_pk_sender,
            pk_receiver: shard_pk_receiver,

            flush_sender: shard_flush_request_sender,
            flush_receiver: shard_flush_complete_receiver,

            close_sender: shard_close_sender,

            compression_sender: shard_compression_sender,
            compression_request_sender: shard_compression_request_sender,
            compression_receiver: shard_compression_receiver,

            encryption_sender: shard_encryption_sender,
            encryption_request_sender: shard_encryption_request_sender,
            encryption_receiver: shard_encryption_receiver,

            cache_supported_sender: shard_cache_supported_sender,
            cache_supported_request_sender: shard_cache_supported_request_sender,
            cache_supported_receiver: shard_cache_supported_receiver,
        }
    }
}

pub struct ConnectionShard {
    pk_sender: broadcast::Sender<GamePackets>,
    pk_receiver: broadcast::Receiver<Result<GamePackets, ConnectionError>>,

    flush_sender: watch::Sender<()>,
    flush_receiver: watch::Receiver<()>,

    close_sender: watch::Sender<()>,

    compression_sender: watch::Sender<Option<Compression>>,
    compression_request_sender: watch::Sender<()>,
    compression_receiver: watch::Receiver<Option<Compression>>,

    encryption_sender: watch::Sender<Option<Encryption>>,
    encryption_request_sender: watch::Sender<()>,
    encryption_receiver: watch::Receiver<Option<Encryption>>,

    cache_supported_sender: watch::Sender<bool>,
    cache_supported_request_sender: watch::Sender<()>,
    cache_supported_receiver: watch::Receiver<bool>,
}

impl ConnectionShard {
    pub async fn send(&mut self, pk: GamePackets) -> Result<(), ConnectionError> {
        match self.pk_sender.send(pk) {
            Ok(_) => Ok(()),
            Err(_) => Err(ConnectionError::ConnectionClosed),
        }
    }

    pub async fn recv(&mut self) -> Result<GamePackets, ConnectionError> {
        self.pk_receiver
            .recv()
            .await
            .unwrap_or_else(|_| Err(ConnectionError::ConnectionClosed))
    }

    pub async fn flush(&mut self) -> Result<(), ConnectionError> {
        match self.flush_sender.send(()) {
            Ok(_) => {}
            Err(_) => return Err(ConnectionError::ConnectionClosed),
        }

        match self.flush_receiver.changed().await {
            Ok(_) => Ok(()),
            Err(_) => Err(ConnectionError::ConnectionClosed),
        }
    }

    pub async fn close(mut self) -> Result<(), ConnectionError> {
        match self.flush().await {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        match self.close_sender.send(()) {
            Ok(_) => { /* has been closed successfully */ }
            Err(_) => { /* has already been closed */ }
        };

        Ok(())
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

    pub async fn set_cache_supported(
        &mut self,
        cache_supported: bool,
    ) -> Result<(), ConnectionError> {
        match self.cache_supported_sender.send(cache_supported) {
            Ok(_) => Ok(()),
            Err(_) => Err(ConnectionError::ConnectionClosed),
        }
    }

    pub async fn get_cache_supported(&mut self) -> Result<bool, ConnectionError> {
        match self.cache_supported_request_sender.send(()) {
            Ok(_) => {}
            Err(_) => return Err(ConnectionError::ConnectionClosed),
        };

        match self.cache_supported_receiver.changed().await {
            Ok(_) => Ok(self
                .cache_supported_receiver
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

            flush_sender: self.flush_sender.clone(),
            flush_receiver: self.flush_receiver.clone(),

            close_sender: self.close_sender.clone(),

            compression_sender: self.compression_sender.clone(),
            compression_request_sender: self.compression_request_sender.clone(),
            compression_receiver: self.compression_receiver.clone(),

            encryption_sender: self.encryption_sender.clone(),
            encryption_request_sender: self.compression_request_sender.clone(),
            encryption_receiver: self.encryption_receiver.clone(),

            cache_supported_sender: self.cache_supported_sender.clone(),
            cache_supported_request_sender: self.cache_supported_request_sender.clone(),
            cache_supported_receiver: self.cache_supported_receiver.clone(),
        }
    }
}
