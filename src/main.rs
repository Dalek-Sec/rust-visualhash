extern crate argon2;
use argon2::Config;

fn main() {
    println!("Hello, world!");
    let hash = hash(b"sup");
    println!("{:?}", hash);
}

fn hash(key: &[u8]) -> Vec<u8> {
    let salt: &[u8] = &[0; 8];
    let config = Config::default();
    let hash = argon2::hash_raw(key, salt, &config).unwrap();
    
    hash
}

