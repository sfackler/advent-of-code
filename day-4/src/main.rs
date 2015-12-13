extern crate openssl;
extern crate hex;

use hex::ToHex;
use openssl::crypto::hash::{Hasher, Type};
use std::io::Write;

fn main() {
    let base = "bgvyzdsv";
    let mut hasher = Hasher::new(Type::MD5);
    for i in 0.. {
        hasher.write_all(format!("{}{}", base, i).as_bytes()).unwrap();
        let hash = hasher.finish().to_hex();

        let count = hash.chars().take_while(|&x| x == '0').count();
        if count >= 5 {
            println!("{}", i);
            break;
        }
    }
}
