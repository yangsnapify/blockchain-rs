use super::Block;
use serde::{ Serialize, Deserialize };
use sha2::{ Sha256, Digest };
use rsa::{ RsaPrivateKey, RsaPublicKey, PaddingScheme };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub signature: Option<String>,
}
