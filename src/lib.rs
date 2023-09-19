mod api;

use api::APIClient;
use clap::{Args, Parser, Subcommand};
use std::error::Error;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encrypt a secret
    Encrypt(EncryptArgs),
    /// Decrypt a secret
    Decrypt(DecryptArgs),
}

#[derive(Args)]
struct EncryptArgs {
    #[arg(short, long, help("Message to encrypt"))]
    secret: String,
    #[arg(short, long, default_value_t=500, help("Secret time to live"))]
    expiry: u64,
}

#[derive(Args)]
struct DecryptArgs {
    #[arg(short, long, help("Room id to decrypt message from"))]
    room_id: String,
}

pub async fn run(args: Cli) -> Result<String, Box<dyn Error>> {
    let client_api = APIClient::new();

    match &args.command {
        Commands::Encrypt(EncryptArgs { secret, expiry }) => {
            let room = client_api
                .submit_secret(secret.to_string(), *expiry)
                .await?;
            let client_response = format!("Secret submited to room with id: {room}");
            Ok(client_response)


        }
        Commands::Decrypt(DecryptArgs { room_id }) => {
            let secret = client_api.get_secret(room_id.to_string()).await?;
            let client_response = format!("Secret found: {secret}");
            Ok(client_response)
        }
    }
}
