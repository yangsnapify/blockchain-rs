use blockchain_rs::{ Transaction, Config, TransactionPool, NonceManager, Blockchain };
use std::sync::{Arc, Mutex};

fn main() {
    let mut chain = Blockchain::new();
    let conf = &Config::load_config(); 
    let key_pair = Transaction::construct_key_pair();
    let nonce_manager = Arc::new(Mutex::new(NonceManager::new()));
    let mut transact_pool = TransactionPool::new(conf.max_transactions_per_pool);

    let mut v1 = Transaction::new("t1", "b", "1 eth", nonce_manager.clone());
    let mut v2 = Transaction::new("t2", "b", "1 eth", nonce_manager.clone());
    let mut v3 = Transaction::new("t3", "b", "1 eth", nonce_manager.clone());
    let mut v4 = Transaction::new("t4", "b", "1 eth", nonce_manager.clone());
    let mut v5 = Transaction::new("t5", "b", "1 eth", nonce_manager.clone());
    let mut v6 = Transaction::new("t6", "b", "1 eth", nonce_manager.clone());
    let mut v7 = Transaction::new("t7", "b", "1 eth", nonce_manager.clone());
    let mut v8 = Transaction::new("t8", "b", "1 eth", nonce_manager.clone());
    let mut v9 = Transaction::new("t8", "b", "1 eth", nonce_manager.clone());

    v1.signature(&key_pair);
    v2.signature(&key_pair);
    v3.signature(&key_pair);
    v4.signature(&key_pair);
    v5.signature(&key_pair);
    v6.signature(&key_pair);
    v7.signature(&key_pair);
    v8.signature(&key_pair);
    v9.signature(&key_pair);

    transact_pool.update_pending_pool(v1, &mut chain);
    transact_pool.update_pending_pool(v2, &mut chain);
    transact_pool.update_pending_pool(v3, &mut chain);
    transact_pool.update_pending_pool(v4, &mut chain);
    transact_pool.update_pending_pool(v5, &mut chain);
    transact_pool.update_pending_pool(v6, &mut chain);
    transact_pool.update_pending_pool(v7, &mut chain);
    transact_pool.update_pending_pool(v8, &mut chain);
    transact_pool.update_pending_pool(v9, &mut chain);
}
