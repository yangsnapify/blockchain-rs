use blockchain_rs::{Transaction, Config, TransactionPool, NonceManager, Blockchain};
use std::sync::{Arc, Mutex};

fn main() {
    // Initialize core components
    let mut blockchain = Blockchain::new();
    let config = Config::load_config();
    let key_pair = Transaction::construct_key_pair();
    let nonce_manager = Arc::new(Mutex::new(NonceManager::new()));
    let mut tx_pool = TransactionPool::new(config.max_transactions_per_pool);

    // Create some sample transactions
    let transactions = vec![
        ("Alice", "Bob", "1.5 ETH"),
        ("Bob", "Charlie", "0.5 ETH"),
        ("Charlie", "Alice", "1.0 ETH"),
    ];

    // Process each transaction
    for (from, to, amount) in transactions {
        // Create and sign transaction
        let mut tx = Transaction::new(from, to, amount, nonce_manager.clone());
        tx.signature(&key_pair);
        
        // Add to pool and process
        tx_pool.update_pending_pool(tx, &mut blockchain);
    }

    // Print the current state of the blockchain
    println!("Current Blockchain State:");
    println!("========================");
    
    for (index, block) in blockchain.chain.iter().enumerate() {
        println!("\nBlock #{}", index);
        println!("Previous Hash: {}", block.previous_hash.as_ref().unwrap_or(&"Genesis Block".to_string()));
        println!("Merkle Tree: {:?}", block.tree);
    }

    // Get some blockchain statistics
    println!("\nBlockchain Statistics:");
    println!("Total Blocks: {}", blockchain.chain.len());
    println!("Pending Transactions: {}", tx_pool.pending_transactions.len());
}