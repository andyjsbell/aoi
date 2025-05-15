//! Oracle CLI Application
//!
//! This binary provides a command-line interface for the Oracle service,
//! which obtains geographical location data, encodes it as a geohash,
//! and cryptographically signs it using Ed25519.
//!
//! # Usage
//!
//! ## Generate a new key pair
//! ```
//! oracle generate
//! ```
//!
//! ## Run the oracle with a specific key and accuracy
//! ```
//! oracle run --key=<hex_key> --accuracy=6
//! ```
//!
//! ## Run using an environment variable for the key
//! ```
//! ORACLE_KEY=<hex_key> oracle run --accuracy=8
//! ```

mod blake2_256;
mod ed25519;
mod env;
mod geohash;

use blake2_256::Blake2_256;
use clap::{Parser, Subcommand};
use ed25519::Ed25519;
use geohash::Geohash;
use oracle::{location, sign_location, Key, Signer};

/// Command-line arguments for the Oracle application.
///
/// This struct represents the top-level CLI arguments,
/// which consist of subcommands for different operations.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The subcommand to execute
    #[command(subcommand)]
    command: Commands,
}

/// Subcommands supported by the Oracle application.
///
/// This enum defines the different operations that can be
/// performed by the application.
#[derive(Subcommand)]
enum Commands {
    /// Generate a new private/public key pair for signing.
    ///
    /// This command generates a new Ed25519 key pair and
    /// outputs both the private and public keys in hexadecimal format.
    Generate,
    
    /// Run the oracle to generate a signed location.
    ///
    /// This command:
    /// 1. Gets the current location
    /// 2. Converts it to a geohash with the specified accuracy
    /// 3. Signs it with the provided key or environment variable
    /// 4. Outputs the signed data as JSON
    Run {
        /// Hexadecimal private key for signing (optional if ORACLE_KEY env var is set).
        ///
        /// The key should be a 32-byte Ed25519 private key in hexadecimal format,
        /// optionally prefixed with "0x".
        #[arg(default_value = "")]
        key: String,
        
        /// Geohash accuracy (1-12), determines precision of location data.
        ///
        /// Higher values provide more precise location data.
        /// Common values:
        /// - 5: City level (~2.4km precision)
        /// - 6: Neighborhood level (~0.61km precision)
        /// - 8: Street level (~38m precision)
        #[arg(default_value = "6")]
        accuracy: u8,
    },
}

/// Main entry point for the Oracle CLI application.
///
/// This function:
/// 1. Parses command-line arguments
/// 2. Executes the requested command (Generate or Run)
/// 3. Handles errors and outputs results
#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Generate => {
            // Generate a new Ed25519 key pair
            let (secret_key, public_key) = Ed25519::generate_key();
            println!(
                "Private=0x{}\nPublic=0x{}",
                env::array_to_hex(secret_key.as_bytes()),
                env::array_to_hex(public_key.as_bytes()),
            );
        }
        Commands::Run { key, accuracy } => {
            // Attempt to get the key from environment variable first, then from command line
            let key_result =
                env::try_key_from_environment().or_else(|_| env::try_hex_to_array(key));

            // Handle key parsing errors gracefully
            let key = match key_result {
                Ok(key_bytes) => Key::new(key_bytes),
                Err(e) => {
                    eprintln!("Error: Failed to get key: {}", e);
                    std::process::exit(1);
                }
            };

            // Get the current location as a geohash
            let location = match location::<Geohash>(accuracy).await {
                Ok(loc) => loc,
                Err(e) => {
                    eprintln!("Error: Failed to get location: {}", e);
                    std::process::exit(1);
                }
            };

            // Sign the location data
            let signed_location = match sign_location::<Geohash, Ed25519, Blake2_256>(key, location).await {
                Ok(sig) => sig,
                Err(e) => {
                    eprintln!("Error: Failed to sign location: {}", e);
                    std::process::exit(1);
                }
            };

            // Output the signed location as JSON
            match serde_json::to_string(&signed_location) {
                Ok(json) => println!("{}", json),
                Err(e) => {
                    eprintln!("Error: Failed to serialize signature: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
