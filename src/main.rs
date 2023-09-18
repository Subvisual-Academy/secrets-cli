use std::env;
use std::process;

use secrets::Secret;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && &args[1] == "help" {
        println!("Usage:");
        println!("  cargo run encrypt <secret>");
        println!("  cargo run decrypt <link_to_secret>");
        process::exit(0);
    }

    let secret = Secret::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = secrets::run(secret) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

