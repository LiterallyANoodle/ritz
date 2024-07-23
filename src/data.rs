use std::fs;

const GIT_DIR: &str = ".ritz";

pub fn init() {
    fs::create_dir_all(GIT_DIR);
}