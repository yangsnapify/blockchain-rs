use blockchain_rs::{ Transaction, Config };

fn main() {
    let key_pair = Transaction::construct_key_pair();
    let mut v1 = Transaction::new("test".to_string(), "b".to_string(), 1, key_pair.public, None);
    v1.signature(&key_pair);
    println!("the sign is {:?} and the signature {:?}", &v1.signature, &v1.signature.as_ref());

    let config = &Config::load_config();
    println!("the max is {}", config.max_transactions_per_pool);
}
