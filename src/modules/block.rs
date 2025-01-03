use crate::modules::{ MerkleTree, Blockchain};

#[derive(Clone, Debug)]
pub struct Block {
    pub timestamp: u64,
    pub tree: MerkleTree,
    pub previous_hash: Option<String>,
}

impl Block {
    pub fn new(tree: MerkleTree, chain: Blockchain) -> Block {
        let _tree = tree.clone();

        let b = Block {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            tree: _tree,
            previous_hash: chain.get_latest()
        };
        b
    }
}