use std::{fs, io::Error};

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

// pub fn hash_object(data) {

// }