use ed25519_dalek::{ Keypair, PublicKey, Signature, Signer, Verifier };
use base64::{ encode, decode };
use rand_core::{ OsRng };

#[derive(Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub public_key: PublicKey,
    pub signature: Option<String>,
}

impl Transaction {
    pub fn new(
        sender: String,
        receiver: String,
        amount: u64,
        public_key: PublicKey,
        signature: Option<String>
    ) -> Self {
        Transaction {
            sender,
            receiver,
            amount,
            public_key,
            signature,
        }
    }

    fn generate_data_string(&self) -> String {
        format!("{}{}{}", self.sender, self.receiver, self.amount)
    }

    pub fn construct_key_pair() -> Keypair {
        let mut rng = OsRng;
        Keypair::generate(&mut rng)
    }

    pub fn signature(&mut self, keypair: &Keypair) {
        let data = self.generate_data_string();
        let signature = keypair.sign(data.as_bytes());
        self.signature = Some(base64::encode(signature.as_ref()));
    }

    pub fn verify(&self) -> bool {
        match &self.signature {
            Some(signature) => {
                let data_string = self.generate_data_string();
                let signature_bytes = decode(signature).expect("Failed to decode signature");
                let value = Signature::from_bytes(&signature_bytes).expect(
                    "Failed to create signature from bytes"
                );
                self.public_key.verify(data_string.as_bytes(), &value).is_ok()
            }
            None => false,
        }
    }
}
