
#[derive(Debug)]
pub struct NonceManager {
    current_nonce: u64,
}

impl NonceManager {
    pub fn new() -> Self {
        NonceManager { current_nonce: 0 }
    }

    pub fn get_and_increment(&mut self) -> u64 {
        let nonce = self.current_nonce;
        self.current_nonce += 1;
        nonce
    }
}