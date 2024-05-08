pub struct Encryption {
    send_counter: u64,
    buf: [u8; 8],
    key: Vec<u8>,
}

impl Encryption {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn decrypt(&mut self, data: &Vec<u8>) -> Vec<u8> {
        unimplemented!()
    }

    pub fn encrypt(&mut self, data: &Vec<u8>) -> Vec<u8> {
        unimplemented!()
    }

    pub fn verify(&mut self, data: &Vec<u8>) -> Result<(), ()> {
        unimplemented!()
    }
}
