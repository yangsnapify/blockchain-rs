use ed25519_dalek::{ Keypair, PublicKey, Signature, Signer, Verifier, Digest };
use base64::{ encode, decode };
use rand_core::{ OsRng };
use crate::modules::{ transaction_pool, NonceManager };
use serde::{ Serialize, Deserialize };
use sha2::Sha256;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: String,
    pub signature: Option<String>,
    pub transaction_id: Option<String>,
    pub nonce: u64,
}

impl Transaction {
    pub fn new(sender: impl Into<String>, receiver: impl Into<String>, amount: impl Into<String>, nonce_manager: Arc<Mutex<NonceManager>>) -> Self {
        let sender: String = sender.into();
        let receiver: String = receiver.into();
        let amount: String = amount.into();

        let mut nonce_manager = nonce_manager.lock().unwrap();
        let nonce = nonce_manager.get_and_increment();

        Transaction {
            sender,
            receiver,
            amount,
            nonce,
            signature: None,
            transaction_id: None,
        }
    }

    fn generate_data_string(&self) -> String {
        format!("{}{}{}", self.sender, self.receiver, self.amount)
    }

    pub fn construct_key_pair() -> Keypair {
        let mut rng = OsRng;
        Keypair::generate(&mut rng)
    }

    fn generate_transaction_id(&self) -> String {
        let transaction_data = format!("{}{}{}{}", self.sender, self.receiver, self.amount, self.nonce);
        let mut hasher = Sha256::new();
        hasher.update(transaction_data.as_bytes());
        let result = hasher.finalize();
        let hex_hash = encode(result);
        format!("0x{}", hex_hash)
    }

    pub fn signature(&mut self, keypair: &Keypair) {
        let data = self.generate_data_string();
        let signature = keypair.sign(data.as_bytes());
        self.signature = Some(encode(signature.as_ref()));
        self.transaction_id = Some(Transaction::generate_transaction_id(self));
    }

    pub fn verify(&self, keypair: &Keypair) -> bool {
        match &self.signature {
            Some(signature) => {
                let data_string = self.generate_data_string();
                let signature_bytes = decode(signature).expect("Failed to decode signature");
                let value = Signature::from_bytes(&signature_bytes).expect(
                    "Failed to create signature from bytes"
                );
                keypair.verify(data_string.as_bytes(), &value).is_ok()
            }
            None => false,
        }
    }

    pub fn hash(&self) -> String {
        let data = format!(
            "{}{}{}{}{}{}",
            self.sender,
            self.receiver,
            self.amount,
            self.nonce,
            self.signature.clone().unwrap_or_default(),
            self.transaction_id.clone().unwrap_or_default()
        );

        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}
