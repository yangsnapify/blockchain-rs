use blockchain_rs::{ Transaction, Config, TransactionPool };

fn main() {
    let conf = &Config::load_config(); 
    let key_pair = Transaction::construct_key_pair();
    let mut transact_pool = TransactionPool::new(conf.max_transactions_per_pool);

    let mut v1 = Transaction::new("t1".to_string(), "a".to_string(), 1, key_pair.public, None);
    let mut v2 = Transaction::new("t2".to_string(), "b".to_string(), 1, key_pair.public, None);

    v1.signature(&key_pair);
    v2.signature(&key_pair);

    transact_pool.update_pending_pool(v1);
    transact_pool.update_pending_pool(v2);
}
