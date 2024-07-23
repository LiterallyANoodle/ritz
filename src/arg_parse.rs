// Submodule for parsing command line arguments
use clap::{Parser, Subcommand};

// struct defining possible arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    #[command(subcommand)]
    cmd: Commands,

}

#[derive(Subcommand, Debug)]
enum Commands {
    
    init,
    commit,

}

pub fn arg_parse() {
    let args = Args::parse();
    dbg!(args);
}