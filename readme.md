# Blockchain Rust Implementation

A lightweight and efficient blockchain implementation in Rust that features transaction management, signature verification, and Merkle tree implementation for block validation.

## Features

- Transaction creation and management
- Cryptographic signature verification
- Merkle tree for transaction validation
- Thread-safe nonce management
- Configurable transaction pool size
- Memory-efficient block storage

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
blockchain_rs = "0.1.0"
```

## Quick Start

```rust
use blockchain_rs::{Transaction, Config, TransactionPool, NonceManager, Blockchain};
use std::sync::{Arc, Mutex};

fn main() {
    // Initialize blockchain components
    let mut blockchain = Blockchain::new();
    let config = Config::load_config();
    let key_pair = Transaction::construct_key_pair();
    let nonce_manager = Arc::new(Mutex::new(NonceManager::new()));
    let mut tx_pool = TransactionPool::new(config.max_transactions_per_pool);

    // Create and sign a transaction
    let mut tx = Transaction::new("sender", "receiver", "1 ETH", nonce_manager.clone());
    tx.signature(&key_pair);
    
    // Add to pool
    tx_pool.update_pending_pool(tx, &mut blockchain);
}
```

## Core Components

### Blockchain
The main data structure that maintains the chain of blocks. Each block contains:
- Previous block hash (Option<String>)
- Merkle tree root
- Timestamp
- Block index

### Transaction
Represents a single transaction in the blockchain:
- Sender address
- Receiver address
- Amount
- Digital signature
- Nonce for ordering

### TransactionPool
Manages pending transactions:
- Configurable pool size
- Transaction validation
- Block creation triggers

### NonceManager
Handles transaction ordering:
- Thread-safe design
- Sequential nonce assignment
- Prevents double-spending

### Creating Multiple Transactions

```rust
let mut tx1 = Transaction::new("Alice", "Bob", "1 eth", nonce_manager.clone());
let mut tx2 = Transaction::new("Bob", "Charlie", "2 eth", nonce_manager.clone());

tx1.signature(&key_pair);
tx2.signature(&key_pair);

transact_pool.update_pending_pool(tx1, &mut chain);
transact_pool.update_pending_pool(tx2, &mut chain);
```

### Viewing Blockchain State

```rust
for block in &chain.chain {
    println!("Block Index: {:?}", block.tree);
    println!("Previous Hash: {}", 
        block.previous_hash.as_ref().unwrap_or(&"Genesis Block".to_string()));
}
```

## Configuration

The blockchain can be configured through the `Config` struct:
- Maximum transactions per pool
- Block size limits

## Configuration

The blockchain is configured through a `block_config.json` file that should be placed in your project's root directory:

```json
{
    "max_transactions_per_pool": 100,
    "block_size_limit": 1024,
}
```

### Configuration Parameters

- `max_transactions_per_pool`: Maximum number of transactions allowed in the memory pool
- `block_size_limit`: Maximum size of a block in bytes

### Loading Configuration

```rust
use blockchain_rs::Config;

fn main() {
    // Load configuration from block_config.json
    let config = Config::load_config();
    
    // Initialize blockchain with config
    let mut blockchain = Blockchain::new();
    let mut tx_pool = TransactionPool::new(config.max_transactions_per_pool);
}
```

## Best Practices

1. Always verify transaction signatures before adding to pool
2. Use proper error handling for failed transactions
3. Implement proper synchronization for multi-threaded access
4. Regularly backup blockchain state
5. Monitor transaction pool size

## Acknowledgments

- Inspired by blockchain fundamentals and best practices
- Built with Rust's safety and performance principles
- Utilizing modern cryptographic standards

## Future Improvements

- [ ] Add consensus mechanism
- [ ] Implement network layer
- [ ] Add smart contract support
- [ ] Enhance transaction validation
- [ ] Add block pruning capabilities