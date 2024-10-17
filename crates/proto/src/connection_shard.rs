use crate::compression::Compression;
use crate::encryption::Encryption;
use crate::error::ConnectionError;
use crate::gamepackets::GamePackets;
use tokio::sync::watch::Ref;
use tokio::sync::{mpsc, watch};

#[derive(Debug, Clone)]
pub struct ConnectionSharedSender {
    pub(super) gamepacket_sender: mpsc::Sender<GamePackets>,

    pub(super) close_sender: watch::Sender<()>,

    pub(super) flush_sender: watch::Sender<()>,

    pub(super) compression_sender: watch::Sender<Option<Compression>>,
    pub(super) compression_receiver: watch::Receiver<Option<Compression>>,

    pub(super) encryption_sender: watch::Sender<Option<Encryption>>,
    pub(super) encryption_receiver: watch::Receiver<Option<Encryption>>,
}

impl ConnectionSharedSender {
    pub async fn send(&mut self, packet: GamePackets) -> Result<(), ConnectionError> {
        self.gamepacket_sender
            .send(packet)
            .await
            .map_err(|_| ConnectionError::ConnectionClosed)?;
        Ok(())
    }

    pub async fn flush(&mut self) -> Result<(), ConnectionError> {
        self.flush_sender
            .send(())
            .map_err(|_| ConnectionError::ConnectionClosed)?;
        Ok(())
    }

    pub fn get_compression(&mut self) -> Result<Option<Compression>, ConnectionError> {
        Ok(self.compression_receiver.borrow().clone())
    }

    pub fn get_compression_ref(&mut self) -> Result<Ref<'_, Option<Compression>>, ConnectionError> {
        Ok(self.compression_receiver.borrow())
    }

    pub fn set_compression(
        &mut self,
        compression: Option<Compression>,
    ) -> Result<(), ConnectionError> {
        self.compression_sender
            .send(compression)
            .map_err(|_| ConnectionError::ConnectionClosed)?;
        Ok(())
    }

    pub fn get_encryption(&mut self) -> Result<Option<Encryption>, ConnectionError> {
        Ok(self.encryption_receiver.borrow().clone())
    }

    pub fn get_encryption_ref(&mut self) -> Result<Ref<'_, Option<Encryption>>, ConnectionError> {
        Ok(self.encryption_receiver.borrow())
    }

    pub fn set_encryption(
        &mut self,
        encryption: Option<Encryption>,
    ) -> Result<(), ConnectionError> {
        self.encryption_sender
            .send(encryption)
            .map_err(|_| ConnectionError::ConnectionClosed)?;
        Ok(())
    }

    pub async fn close(self) {
        // Already closed if Error occurs
        let _ = self.close_sender.send(());
    }
}

#[derive(Debug)]
pub struct ConnectionSharedReceiver {
    pub(super) gamepacket_receiver: mpsc::Receiver<Result<GamePackets, ConnectionError>>,

    pub(super) close_sender: watch::Sender<()>,

    pub(super) compression_receiver: watch::Receiver<Option<Compression>>,
    pub(super) encryption_receiver: watch::Receiver<Option<Encryption>>,
}

impl ConnectionSharedReceiver {
    pub async fn recv(&mut self) -> Result<GamePackets, ConnectionError> {
        self.gamepacket_receiver
            .recv()
            .await
            .unwrap_or_else(|| Err(ConnectionError::ConnectionClosed))
    }

    pub fn get_compression(&mut self) -> Result<Option<Compression>, ConnectionError> {
        Ok(self.compression_receiver.borrow().clone())
    }

    pub fn get_compression_ref(&mut self) -> Result<Ref<'_, Option<Compression>>, ConnectionError> {
        Ok(self.compression_receiver.borrow())
    }

    pub fn get_encryption(&mut self) -> Result<Option<Encryption>, ConnectionError> {
        Ok(self.encryption_receiver.borrow().clone())
    }

    pub fn get_encryption_ref(&mut self) -> Result<Ref<'_, Option<Encryption>>, ConnectionError> {
        Ok(self.encryption_receiver.borrow())
    }

    pub async fn close(self) {
        // Already closed if Error occurs
        let _ = self.close_sender.send(());
    }
}
