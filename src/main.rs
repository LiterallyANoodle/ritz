extern crate clap;
use clap::{Parser, Subcommand};

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

}

fn ritz_init(args: Args) {
    println!("Init not implemented!");
}

fn ritz_commit(args: Args) {
    println!("Commit not implemented!");
}

fn ritz_default(args: Args) {
    println!("Please use a subcommand: init, commit");
}

fn main() {
    println!("Hello, world!");
    let args = Args::parse();

    match args.cmd {
        Some(ref c) => {
            match c {
                Commands::init => { ritz_init(args) },
                Commands::commit => { ritz_commit(args) },
                _ => { ritz_default(args) },
            }
        },
        None => {
            ritz_default(args)
        }
    }
}
