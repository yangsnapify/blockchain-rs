use crate::modules::{ Transaction, Block, MerkleTree, Blockchain};

pub struct TransactionPool {
    pub pending_transactions: Vec<Transaction>,
    pub max_transactions_per_pool: usize,
}

impl TransactionPool {
    pub fn new(max_size: usize) -> Self {
        Self {
            max_transactions_per_pool: max_size,
            pending_transactions: Vec::new(),
        }
    }

    pub fn update_pending_pool(&mut self, data: Transaction, chain: &mut Blockchain) {
        if self.max_transactions_per_pool == 0 {
            println!("Error: max_transactions_per_pool must be greater than 0!");
            return;
        }
        if self.pending_transactions.len() >= self.max_transactions_per_pool {
            self.commit_to_block(chain);
            self.pending_transactions.clear();
        }
        self.pending_transactions.push(data);
    }

    pub fn print_pool_size(&self) {
        println!("Pending transactions: {}", self.pending_transactions.len());
    }

    pub fn commit_to_block(&mut self, chain: &mut Blockchain) {
        let val = self.pending_transactions.clone();
        let tree = MerkleTree::new(val.clone());
        let chain_cloned = chain.clone();
    
        let block_create = Block::new(tree, chain_cloned);
        chain.commit_to_chain(block_create);
        self.pending_transactions.clear();
    }
}
