use blockchain_rs::{ Block, Transaction };

fn main() {
    let key_pair = Transaction::construct_key_pair();
    let mut v1 = Transaction::new("test".to_string(), "b".to_string(), 1, key_pair.public, None);
    v1.signature(&key_pair);
    println!("the sign is {} and the signature {:?}", v1.verify(), &v1.signature);
}
