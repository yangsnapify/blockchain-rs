use crate::modules::{ Block };

#[derive(Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            chain: Vec::new(),
        }
    }
    pub fn get_latest(&self) -> Option<String> {
        self.chain.last().map(|block| block.tree.root.clone())
    }
    pub fn commit_to_chain(&mut self, block_data: Block) {
        self.chain.push(block_data);
    }
}