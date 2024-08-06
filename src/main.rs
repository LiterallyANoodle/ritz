extern crate clap;
mod data;
use clap::{Parser, Subcommand};
use std::{fs::File, io, io::prelude::*};

// struct defining possible arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {

    #[command(subcommand)]
    cmd: Option<Commands>,

}

#[derive(Subcommand, Debug)]
enum Commands {
    
    init,
    commit,
    read_test,
    sha_test,

}

fn ritz_init(args: Args) {
    data::init();
}

fn ritz_commit(args: Args) {
    println!("Commit not implemented!");
}

fn ritz_default(args: Args) {
    println!("Please use a subcommand: init, commit");
}

fn read_file_as_bytes(path: &str) -> io::Result<Vec<u8>> {
    let mut f = File::open(path)?;
    let mut buffer = Vec::new();
    
    f.read_to_end(&mut buffer)?;
    return Ok(buffer);
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Some(ref c) => {
            match c {
                Commands::init => { ritz_init(args) },
                Commands::commit => { ritz_commit(args) },
                Commands::read_test => { 
                    let buffer = read_file_as_bytes("test.txt");
                    println!("Here come the test results: {buffer:?}");
                    return (); },
                Commands::sha_test => { data::hash_object(); }
                _ => { ritz_default(args) },
            }
        },
        None => {
            ritz_default(args)
        }
    }
}
