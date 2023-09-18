use std::error::Error;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short,long, help("Message to encrypt"))]
    pub encrypt: Option<String>, 
    #[arg(short,long, help("Link to decrypt message from"))]
    pub decrypt: Option<String>,
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
   if let Some(message) = args.encrypt {
       println!("Encrypting {message}");
   }

   if let Some(link) = args.decrypt {
       println!("Decrypting from {link}");
   }

   Ok(())

}
