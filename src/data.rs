use std::{fs, io::Error};
use sha1::{Sha1, Digest};
// use hex_literal::hex;
use hex::{encode, decode};

const GIT_DIR: &str = ".ritz";

pub fn init() {
    if (std::path::Path::new(GIT_DIR).exists()) {
        println!("Already a ritz repository.");
        return;
    }
    match fs::create_dir_all(GIT_DIR) {
        Ok(_) => (),
        Err(e) => panic!("{e:?}"),
    }
    match fs::create_dir_all(format!("{GIT_DIR}\\objects")) {
        Ok(_) => (),
        Err(e) => panic!("{e:?}"),
    }    
    println!("Successfully initialized ritz repository.");
}

pub fn hash_object() {
    let mut hasher = Sha1::new();
    hasher.update(b"Hello world!");
    let result = hasher.finalize();
    let resultStr = encode(result);
    // assert_eq!(result[..], encode("2aae6c35c94fcfb415dbe95f408b9ce91ee846ed"));
    println!("{resultStr:?}");
}