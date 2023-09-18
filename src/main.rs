use std::process;
use clap::Parser;
use secrets::Args;


fn main() {
    let args = Args::parse(); 
    
    if let Err(e) = secrets::run(args) {
         eprintln!("Application error: {e}");
         process::exit(1);
    }
}

