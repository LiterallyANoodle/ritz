use std::{fs, io::Error};

const GIT_DIR: &str = ".ritz";

pub fn init() {
    if (std::path::Path::new(GIT_DIR).exists()) {
        println!("Already a ritz repository.");
        return;
    }
    match fs::create_dir_all(GIT_DIR) {
        Ok(_) => println!("Successfully initialized ritz repository."),
        Err(e) => println!("{:?}", e),
    }
}