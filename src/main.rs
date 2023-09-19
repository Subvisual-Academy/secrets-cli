use clap::Parser;
use secrets::Cli;
use std::process;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let args = Cli::parse();

    match secrets::run(args).await {
        Err(e) => {
            eprintln!("Aplication error: {e}");
            process::exit(1);
        }
        Ok(response) => {
            println!("{response}");
        }
    }
}
