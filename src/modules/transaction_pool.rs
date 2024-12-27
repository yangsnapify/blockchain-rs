use crate modules::{{Transaction, Block, Config}};

pub struct TransactPool {
    pending_transactions: Vec!<Transaction>;
}

impl TransactPool {
    pub fn update_pending_pool(&mut self, data: Transaction) {
        let config = Config::load_config();
        let max_transactions = config.max_transactions_per_pool;
        if self.pending_transactions.len() > max_transactions_per_pool {
            self.commit_to_block();
            self.pending_transactions.push(data);
        } else {
            self.pending_transactions.push(data);
        }
    }

    pub fn print_pool_size(&self) {
        println!("Pending transactions: {}", self.pending_transaction.len());
    }

    pub fn commit_to_block(&mut self) {
        // let block_create = Block::new();

        self.pending_transactions.clear();
    }
}