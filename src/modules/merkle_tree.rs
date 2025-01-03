use sha2::{Sha256, Digest};
use crate::modules::{ Transaction };

#[derive(Clone, Debug)]
pub struct MerkleTree {
    pub root: String,
    pub leaves: Vec<String>,
}

impl MerkleTree {
    pub fn new(transactions: Vec<Transaction>) -> Self {
        let leaves: Vec<String> = transactions.into_iter().map(|tx| tx.hash()).collect();
        let root = MerkleTree::build_trees(leaves.clone());

        MerkleTree { root, leaves }
    }

    fn build_trees(mut hashes: Vec<String>) -> String {
        while hashes.len() > 1 {
            let mut new_hashes = Vec::new();

            for i in (0..hashes.len()).step_by(2) {
                if i + 1 < hashes.len() {
                    let combined = format!("{}{}", hashes[i], hashes[i + 1]);
                    let mut hasher = Sha256::new();
                    hasher.update(combined.as_bytes());
                    new_hashes.push(format!("{:x}", hasher.finalize()));
                } else {
                    new_hashes.push(hashes[i].clone());
                }
            }
            println!("{:?}", &new_hashes);
            hashes = new_hashes;
        }
        hashes[0].clone()
    }

    pub fn get_merkle_root(&self) -> &str {
        &self.root
    }

    pub fn get_leaves(&self) -> &[String] {
        &self.leaves
    }
}

