mod blake2_256;
mod ed25519;
mod env;
mod geohash;

use blake2_256::Blake2_256;
use clap::{Parser, Subcommand};
use ed25519::Ed25519;
use geohash::Geohash;
use oracle::{Key, Signer, location, sign_location};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new private key
    Generate,
    /// Run with a specific key and accuracy
    Run {
        #[arg(default_value = "")]
        key: String,
        #[arg(default_value = "6")]
        accuracy: u8,
    },
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Generate => {
            let (secret_key, public_key) = Ed25519::generate_key();
            println!(
                "Private=0x{}\nPublic=0x{}",
                env::array_to_hex(secret_key.as_bytes()),
                env::array_to_hex(public_key.as_bytes()),
            );
        }
        Commands::Run { key, accuracy } => {
            let key = Key::new(
                env::try_key_from_environment()
                    .or_else(|_| env::try_hex_to_array(key))
                    .expect("a well formed hexadecimal string for the key"),
            );

            let location = location::<Geohash>(accuracy).await.expect("valid location");
            let signed_location = sign_location::<Geohash, Ed25519, Blake2_256>(key, location)
                .await
                .expect("signed location");

            println!(
                "{}",
                serde_json::to_string(&signed_location).expect("valid json")
            );
        }
    }
}
