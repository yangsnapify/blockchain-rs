use sha2::{Sha256, Digest};
use hex;

#[derive(Clone)]
struct MerkleNode {
    value: String,
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>,
}

impl MerkleNode {
    fn new(value: String) -> Self {
        MerkleNode {
            value,
            left: None,
            right: None,
        }
    }
}

pub struct MerkleTree {
    root: Option<MerkleNode>,
    leaves: Vec<MerkleNode>,
}

