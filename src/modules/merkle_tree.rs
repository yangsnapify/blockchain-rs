use sha2::{Sha256, Digest};
use crate::modules::{ Transaction };

#[derive(Clone)]
pub struct MerkleNode {
    val: String,
    id: Option<String>,
    parent: Option<String>,
    left: Option<String>,
    right: Option<String>,
}

impl MerkleNode {
    fn new(value: String) -> Self {
        MerkleNode {
            val: value,
            id: None,
            parent: None,
            left: None,
            right: None,
        }
    }
}

pub struct MerkleTree {
    root: Option<MerkleNode>,
    leaves: Vec<MerkleNode>,
}

impl MerkleTree {
    fn new(values: Vec<Transaction>) {
        
    }
}

