use crate::modules::{Transaction, MerkleTree};

pub struct Block {
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(transactions: Vec<Transaction>, previous_hash: String) {
        let cloned_transactions = transactions.clone();
        // let merkle_tree = MerkleTree::new(transactions);
        // Block {
        //     timestamp: std::time::SystemTime::now()
        //         .duration_since(std::time::UNIX_EPOCH)
        //         .unwrap()
        //         .as_secs(),
        //     transactions: cloned_transactions,
        //     previous_hash,
        //     hash: "".to_string(),
        // }
    }
}