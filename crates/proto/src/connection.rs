use crate::compression::Compression;
use crate::encryption::Encryption;
use crate::error::ConnectionError;
use crate::gamepackets::GamePackets;
use crate::sub_client::SubClientID;
use crate::transport_layer::TransportLayerConnection;
use std::io::Cursor;
use std::time::Duration;
use tokio::select;
use tokio::sync::{broadcast, watch};
use tokio::time::interval;

pub struct Connection {
    /// Represents the Connection's internal transport layer, which may vary.
    transport_layer: TransportLayerConnection,
    /// Represents the Connection's Compression, the compression gets initialized in the
    /// login process.
    pub compression: Option<Compression>,
    /// Represents the connections encryption, the encryption gets initialized in the
    /// login process, if encryption is enabled.
    pub encryption: Option<Encryption>,
    /// Determines if the Connection does support caching.
    pub cache_supported: bool,
}

impl Connection {
    pub(crate) fn from_transport_conn(transport_layer: TransportLayerConnection) -> Self {
        Self {
            transport_layer,
            compression: None,
            encryption: None,
            cache_supported: false,
        }
    }

    pub async fn send(&mut self, gamepackets: &[GamePackets]) -> Result<(), ConnectionError> {
        let gamepacket_stream_size = gamepackets
            .iter()
            .map(GamePackets::get_size_prediction)
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
        if let Some(compression) = &self.compression {
            gamepacket_stream = compression.compress(gamepacket_stream)?;
        }

        // Encrypt the stream with the given optional Encryption
        if let Some(encryption) = &mut self.encryption {
            gamepacket_stream = encryption.encrypt(gamepacket_stream)?;
        }

        self.transport_layer.send(&gamepacket_stream).await?;

        Ok(())
    }

    pub async fn send_raw(&mut self, data: &[u8]) -> Result<(), ConnectionError> {
        self.transport_layer.send(data).await?;

        Ok(())
    }

    pub async fn recv(&mut self) -> Result<Vec<GamePackets>, ConnectionError> {
        let mut gamepacket_stream = self.transport_layer.recv().await?;

        // Decrypt the stream with the given optional Encryption
        if let Some(encryption) = &mut self.encryption {
            gamepacket_stream = encryption.decrypt(gamepacket_stream)?;
        }

        // Decompress the stream with the given optional Compression
        if let Some(compression) = &self.compression {
            gamepacket_stream = compression.decompress(gamepacket_stream)?;
        }

        let mut gamepacket_stream = Cursor::new(gamepacket_stream.as_slice());
        let mut gamepackets = vec![];

        loop {
            if gamepacket_stream.position() == gamepacket_stream.get_ref().len() as u64 {
                break;
            }

            gamepackets.push(GamePackets::pk_deserialize(&mut gamepacket_stream)?.0);
        }

        Ok(gamepackets)
    }

    pub async fn recv_raw(&mut self) -> Result<Vec<u8>, ConnectionError> {
        let stream = self.transport_layer.recv().await?;

        Ok(stream)
    }

    pub async fn close(self) {
        self.transport_layer.close().await;
    }

    pub async fn into_shard(
        mut self,
        flush_interval: Duration,
        packet_buffer_size: usize,
    ) -> ConnectionShard {
        let (sd_pk_send, mut tk_pk_recv) = broadcast::channel::<GamePackets>(packet_buffer_size);
        let (tk_pk_send, sh_pk_recv) =
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
                                    if tk_pk_send.send(Ok(pk)).is_err() {
                                        break 'select_loop
                                    }
                                }
                            }
                            Err(e) => {
                                if tk_pk_send.send(Err(e)).is_err() {
                                    break 'select_loop
                                }
                            }
                        }
                    }
                    res = tk_pk_recv.recv() => {
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

            self.transport_layer.close().await;
        });

        ConnectionShard {
            pk_sender: sd_pk_send,
            pk_receiver: sh_pk_recv,

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
