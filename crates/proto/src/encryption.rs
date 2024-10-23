use bedrockrs_proto_core::error::EncryptionError;

#[derive(Debug, Clone)]
pub struct Encryption {
    send_counter: u64,
    buf: [u8; 8],
    key: Vec<u8>,
}

impl Encryption {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn decrypt(&mut self, _src: Vec<u8>) -> Result<Vec<u8>, EncryptionError> {
        unimplemented!()
    }

    pub fn encrypt(&mut self, _src: Vec<u8>) -> Result<Vec<u8>, EncryptionError> {
        unimplemented!()
    }

    pub fn verify(&mut self, _src: &[u8]) -> Result<(), EncryptionError> {
        unimplemented!()
    }
}
